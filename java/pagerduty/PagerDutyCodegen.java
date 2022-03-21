package pagerduty;

import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Stream;

import com.github.jknack.handlebars.Handlebars;

import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenModelFactory;
import io.swagger.codegen.v3.CodegenModelType;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.SupportingFile;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.BooleanSchema;
import io.swagger.v3.oas.models.media.ComposedSchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.oas.models.responses.ApiResponse;
import io.swagger.v3.core.util.Json;

public class PagerDutyCodegen extends RustServerCodegen {
    private static final Logger LOGGER = LoggerFactory.getLogger(PagerDutyCodegen.class);

    protected Map<String, CodegenTag> tagList = new HashMap<String, CodegenTag>();
    protected Map<String, List<CodegenProperty>> mapLikeModels = new HashMap<String, List<CodegenProperty>>();

    public PagerDutyCodegen() {
        super();

        additionalProperties.put("tags", tagList.values());
        supportingFiles.add(new SupportingFile("lib.mustache", "src", "lib.rs"));
    }

    private static HashMap<String, Object> patchOperationBodyNames = new HashMap();
    private static HashMap<String, Object> patchOperationResponseNames = new HashMap();
    private static HashMap<String, HashMap<String, Object>> patchOneOfProperties = new HashMap();

    @Override
    public void preprocessOpenAPI(OpenAPI openAPI) {
        Info info = openAPI.getInfo();
        List versionComponents = new ArrayList(Arrays.asList(info.getVersion().split("[.]")));
        while (versionComponents.size() < 3) {
            // Add the package version as a version component to the official specification
            // version
            versionComponents.add((String) additionalProperties.get(CodegenConstants.PACKAGE_VERSION));
        }

        info.setVersion(StringUtils.join(versionComponents, "."));

        super.preprocessOpenAPI(openAPI);
    }

    @Override
    public String getTypeDeclaration(Schema p) {
        String type = super.getTypeDeclaration(p);

        // This is a "fallback" type, and allows some parts of the API
        // that receive an empty JSON '{}' value.
        if ("object".equals(type)) {
            type = "HashMap<String, Value>";
        }

        return type;
    }

    @Override
    public CodegenModel fromModel(String name, Schema schema, Map<String, Schema> allDefinitions) {
        if (name.equals("inline_response_200_11")) {
            final CodegenModel codegenModel = CodegenModelFactory.newInstance(CodegenModelType.MODEL);
            if (reservedWords.contains(name)) {
                codegenModel.name = escapeReservedWord(name);
            } else {
                codegenModel.name = name;
            }
            codegenModel.title = escapeText(schema.getTitle());
            codegenModel.description = escapeText(schema.getDescription());
            codegenModel.unescapedDescription = schema.getDescription();
            codegenModel.classname = toModelName(name);
            codegenModel.classVarName = toVarName(name);
            codegenModel.classFilename = toModelFilename(name);
            codegenModel.modelJson = Json.pretty(schema);
            codegenModel.externalDocumentation = schema.getExternalDocs();
            return codegenModel;
        }

        CodegenModel mdl = super.fromModel(name, schema, allDefinitions);

        // Partially deal with inline object polymorphism: 'anyOf' and 'oneOf' 
        if (schema instanceof ComposedSchema) {
            ComposedSchema composedSchema = (ComposedSchema) schema;
            if (composedSchema.getAllOf() != null) {
                List<Schema> schemas;
                schemas = composedSchema.getAllOf();
                for (Schema subSchema : schemas) {
                    String type = getTypeDeclaration(subSchema);

                    // Deal with recursive 'allOf' in Reference types
                    if (type.equals("Tag/allOf/0") || type.equals("Reference")) {
                        Schema refSchema = null;
                        String ref = io.swagger.codegen.v3.generators.util.OpenAPIUtil.getSimpleRef("Tag");
                        if (allDefinitions != null) {
                            refSchema = allDefinitions.get(ref);
                            if (refSchema instanceof ComposedSchema) {
                                final ComposedSchema refComposed = (ComposedSchema) refSchema;
                                final List<Schema> allOf = refComposed.getAllOf();
                                if (allOf != null && !allOf.isEmpty()) {
                                    Schema interfaceSchema = allOf.get(0);
                                    if (interfaceSchema.getProperties() != null) {

                                        List<Map.Entry<String, Schema>> propertyList = new ArrayList<Map.Entry<String, Schema>>(
                                                interfaceSchema.getProperties().entrySet());
                                        final int totalCount = propertyList.size();
                                        for (int i = 0; i < totalCount; i++) {
                                            Map.Entry<String, Schema> entry = propertyList.get(i);

                                            final String key = entry.getKey();
                                            final Schema propertySchema = entry.getValue();

                                            if (mdl.vars.stream().filter(p -> p.baseName.equals(key))
                                                    .collect(java.util.stream.Collectors.toList()).isEmpty()) {
                                                final CodegenProperty codegenProperty = fromProperty(key,
                                                        propertySchema);
                                                mdl.vars.add(0, codegenProperty);
                                                mdl.requiredVars.add(0, codegenProperty);
                                            }
                                        }


                                    }
                                }
                            }
                        }
                    }

                    // Tag pagination models with their inner data type
                    if (type.equals("Pagination")) {
                        for (CodegenProperty item : mdl.vars) {
                            if (item.containerType != null && item.containerType.equals("array")) {
                                if (item.items.datatype.startsWith("Incident")) {
                                    LOGGER.info( " === " + mdl.classname + ", " + patchOperationResponseNames.get(mdl.classname) + " - " + item.items.datatype);
                                }
                                mdl.getVendorExtensions().put("x-codegen-pagination-response-inner", item.items.datatype);
                            }
                        }
                    }
                }
            }
            if (composedSchema.getOneOf() != null || composedSchema.getAnyOf() != null) {
                List<Schema> schemas;
                if (composedSchema.getOneOf() != null) {
                    schemas = composedSchema.getOneOf();
                } else {
                    schemas = composedSchema.getAnyOf();
                }
                mdl.getVendorExtensions().put("x-rustgen-enum-one-of", "true");

                int i = 0;

                Map<String, Object> allowableValues = new HashMap<String, Object>();
                List<CodegenProperty> subModels = (List<CodegenProperty>) new ArrayList();
                allowableValues.put("count", schemas.size());
                Boolean allDisplayableTypes = true;

                for (Schema subSchema : schemas) {
                    String subName = name + "_sub_" + i;
                    CodegenProperty subMdl = fromProperty(subName, subSchema);
                    String type = getTypeDeclaration(subSchema);
                    if (subSchema instanceof ArraySchema) {

                        final ArraySchema arraySchema = (ArraySchema) subSchema;
                        Schema inner = arraySchema.getItems();
                        if (inner == null) {
                            LOGGER.warn("warning!  No inner type supplied for array parameter \"" + subMdl.getName()
                                    + "\", using String");
                            inner = new StringSchema().description("//TODO automatically added by swagger-codegen");
                            arraySchema.setItems(inner);

                        }

                        CodegenProperty item = fromProperty("inner", inner);
                        if (item != null && !getSchemaType(inner).equals("object")) {
                            item.setDatatype(toModelName(item.getDatatype()));
                        }
                        subMdl.items = item;
                        updatePropertyForArray(subMdl, item);

                        subMdl.getVendorExtensions().put(CodegenConstants.IS_LIST_CONTAINER_EXT_NAME, Boolean.TRUE);
                        subMdl.getVendorExtensions().put(CodegenConstants.IS_CONTAINER_EXT_NAME, Boolean.TRUE);

                        allDisplayableTypes = false;
                    } else if (subSchema.getProperties() != null) {

                        if (isObjectSchema(subSchema)) {
                            subMdl.getVendorExtensions().put("x-is-object", Boolean.TRUE);
                        }

                        Map<String, Schema> properties = subSchema.getProperties();
                        Iterator<Schema> values = properties.values().iterator();

                        // We concern ourselves with the first type in an object - this is probably
                        // wrong for more complex definitions.
                        if (values.hasNext()) {
                            subMdl.getVendorExtensions().put(CodegenConstants.IS_MAP_CONTAINER_EXT_NAME, Boolean.TRUE);
                            subMdl.getVendorExtensions().put(CodegenConstants.IS_CONTAINER_EXT_NAME, Boolean.TRUE);
                            Schema innerSchema = values.next();
                            CodegenProperty cp = fromProperty("inner", innerSchema);

                            updatePropertyForMap(subMdl, cp);
                        }
                        allDisplayableTypes = false;
                    } else if (!(subSchema instanceof NumberSchema) && !(subSchema instanceof IntegerSchema)
                            && !(subSchema instanceof StringSchema) && type != null) {
                        allDisplayableTypes = false;
                        subMdl.datatype = toModelName(type);
                    }

                    // Don't re-add a type that's a duplicate (the use of Value can mean we get
                    // dups)
                    Boolean containsDatatype = false;
                    for (CodegenProperty prop : subModels) {
                        if (prop.datatype.equals(subMdl.datatype)) {
                            containsDatatype = true;
                        }
                    }

                    if (!containsDatatype) {
                        subModels.add(subMdl);
                        i++;
                    }
                }

                // Some enums are used when generating a url, so they need to implement
                // std::Display
                if (allDisplayableTypes) {
                    mdl.getVendorExtensions().put("x-rustgen-is-display", Boolean.TRUE);
                }

                mdl.setAllowableValues(allowableValues);
                allowableValues.put("values", subModels);

            }
        }

        return mdl;
    }

    @Override
    public CodegenProperty fromProperty(String name, Schema p) {
        CodegenProperty property = super.fromProperty(name, p);

        // Remove extraneous references
        if (property.datatype.startsWith("models::")) {
            property.datatype = property.datatype.replace("models::", "");
        }

        // Deal with Map-like Models
        if (p instanceof MapSchema) {
            MapSchema mp = (MapSchema) p;
            Object inner = mp.getAdditionalProperties();
            if (!(inner instanceof Schema) && (Boolean) inner && mp.getProperties() != null
                    && !mapLikeModels.containsKey(name)) {
                Map<String, Schema> props = mp.getProperties();
                List<CodegenProperty> listProps = new ArrayList<CodegenProperty>();
                for (Entry<String, Schema> entry : props.entrySet()) {
                    CodegenProperty prop = fromProperty(entry.getKey(), entry.getValue());
                    if ((prop.dataFormat == null || !prop.dataFormat.equals("date-time"))
                            && !languageSpecificPrimitives.contains(prop.datatype)) {
                        prop.datatype = toModelName(prop.datatype);
                    }
                    listProps.add(prop);
                }
                mapLikeModels.put(name, listProps);
            }
        }

        // Deal with OneOf and AnyOf schemas in model properties.
        // We store the enum values as parseable untagged enums in a vendorExtension.
        // This currently only supports plain OneOf, AnyOf and Vectors of both.
        ComposedSchema composedSchema = null;
        if (p instanceof ArraySchema) {
            final ArraySchema arraySchema = (ArraySchema) p;
            Schema inner = arraySchema.getItems();
            if (inner instanceof ComposedSchema) {
                composedSchema = (ComposedSchema) inner;
            }
        }

        if (p instanceof ComposedSchema) {
            composedSchema = (ComposedSchema) p;
        }

        if (composedSchema != null && (composedSchema.getOneOf() != null || composedSchema.getAnyOf() != null)) {

            List<Schema> schemas;
            if (composedSchema.getOneOf() != null) {
                schemas = composedSchema.getOneOf();
            } else {
                schemas = composedSchema.getAnyOf();
            }

            int i = 0;

            Map<String, Object> allowableValues = new HashMap<String, Object>();
            List<CodegenProperty> subModels = (List<CodegenProperty>) new ArrayList();
            allowableValues.put("count", schemas.size());

            for (Schema subSchema : schemas) {
                String subName = name + "_sub_" + i;
                CodegenProperty subMdl = fromProperty(subName, subSchema);
                String type = getTypeDeclaration(subSchema);

                if (!(subSchema instanceof BooleanSchema) && !(subSchema instanceof ArraySchema)
                        && !(subSchema instanceof ComposedSchema) && !(subSchema instanceof MapSchema)
                        && !(subSchema instanceof NumberSchema) && !(subSchema instanceof IntegerSchema)
                        && !(subSchema instanceof StringSchema) && type != null) {
                    if (isObjectSchema(subSchema) && !type.startsWith("HashMap")) {
                        subMdl.datatype = toModelName(type);
                    }
                }

                // Don't re-add a type that's a duplicate (the use of Value can mean we get
                // dups)
                Boolean containsDatatype = false;
                for (CodegenProperty prop : subModels) {
                    if (prop.datatype.equals(subMdl.datatype)) {
                        containsDatatype = true;
                    }
                }

                if (!containsDatatype) {
                    subModels.add(subMdl);
                    i++;
                }

            }

            allowableValues.put("values", subModels);
            property.vendorExtensions.put("x-codegen-one-of-schema", allowableValues);

        }

        return property;
    }

    @Override
    public Map<String, Object> postProcessAllModels(Map<String, Object> objs) {
        Map<String, Object> newObjs = super.postProcessAllModels(objs);

        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allModels = new HashMap<String, CodegenModel>();
        for (Entry<String, Object> entry : objs.entrySet()) {
            String modelName = toModelName(entry.getKey());
            Map<String, Object> inner = (Map<String, Object>) entry.getValue();
            List<Map<String, Object>> models = (List<Map<String, Object>>) inner.get("models");
            for (Map<String, Object> mo : models) {
                CodegenModel cm = (CodegenModel) mo.get("model");
                allModels.put(modelName, cm);

                // Parse out OneOf and AnyOf enum values from a property. We need to do this
                // here, because the affected models need to know their enum values.
                for (CodegenProperty prop : cm.vars) {
                    if (prop.vendorExtensions.get("x-codegen-one-of-schema") != null) {
                        if (prop.getItems() != null) {
                            patchOneOfProperties.put(prop.getItems().datatype,
                                    (HashMap<String, Object>) prop.vendorExtensions.get("x-codegen-one-of-schema"));
                        } else {
                            patchOneOfProperties.put(prop.datatype,
                                    (HashMap<String, Object>) prop.vendorExtensions.get("x-codegen-one-of-schema"));
                        }
                    }
                }
            }
        }

        for (Entry<String, CodegenModel> entry : allModels.entrySet()) {
            CodegenModel model = entry.getValue();
            if (model.getDataType() != null && model.getDataType().equals("boolean")) {
                model.vendorExtensions.put("x-rustgen-is-bool", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("integer")) {
                model.vendorExtensions.put("x-rustgen-is-integer", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("String")) {
                model.vendorExtensions.put("x-rustgen-is-string", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("DateTime")) {
                model.vendorExtensions.put("x-rustgen-is-datetime", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.arrayModelType != null) {
                model.vendorExtensions.put("x-rustgen-is-array", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getIsEnum()) {
                model.vendorExtensions.put("is-enum", true);
            }

            // Patch in the enum values for a OneOf or AnyOf schema found in a previous
            // model property.
            if (model.classname != null) {
                if (model.classname.startsWith("OneOf") || model.classname.startsWith("AnyOf")) {
                    if (patchOneOfProperties.containsKey(model.classname)) {
                        model.allowableValues = patchOneOfProperties.get(model.classname);
                        model.getVendorExtensions().put("x-rustgen-enum-one-of", "true");
                    } else {
                        newObjs.remove(model.classname);
                    }
                }
            }

            for (CodegenProperty prop : model.vars) {
                if (prop.dataFormat != null && prop.dataFormat.equals("dateTime")) {
                    // set DateTime format on properties where appropriate
                    prop.datatype = "DateTime<Utc>";
                }

                if (getBooleanValue(prop, CodegenConstants.IS_ENUM_EXT_NAME)) {
                    ArrayList<HashMap<String, String>> vars = (ArrayList<HashMap<String, String>>) prop.allowableValues
                            .get("enumVars");
                    for (HashMap<String, String> enumVar : vars) {
                        String enumValue = enumVar.get("value");

                        // ensure we can deserialize inline enum values encoded as empty strings
                        if (enumValue != null && enumValue.length() <= 2) {
                            prop.vendorExtensions.put("x-rustgen-has-empty-enum", true);
                        }
                    }
                }

                if (prop.baseName.equals(prop.name)) {
                    prop.vendorExtensions.put("x-rustgen-serde-no-rename", true);
                }
                if (prop.datatype != null && prop.datatype.equals("String")) {
                    prop.vendorExtensions.put("x-rustgen-is-string", true);
                    model.vendorExtensions.put("x-rustgen-has-string", true);
                }

                if (prop.baseName.equals("score") && prop.datatype.equals("i64")) {
                    prop.datatype = "f32";
                }

            }

            if (mapLikeModels.containsKey(model.name)) {
                model.vars = mapLikeModels.get(model.name);
            }
        }

        return newObjs;
    }

    @Override
    public void addOperationToGroup(String tag, String resourcePath, Operation operation, CodegenOperation co,
            Map<String, List<CodegenOperation>> operations) {

        super.addOperationToGroup(tag, resourcePath, operation, co, operations);
        CodegenTag codegenTag;
        if (tagList.containsKey(tag)) {
            codegenTag = tagList.get(tag);
        } else {
            codegenTag = new CodegenTag();
            codegenTag.baseName = underscore(tag);
            codegenTag.classname = tag;
            codegenTag.operations = new ArrayList<CodegenOperation>();
            tagList.put(tag, codegenTag);
        }

        List<CodegenOperation> codegenOperations = codegenTag.getOperations();
        codegenOperations.add(co);
        codegenTag.setContents(co.getContents());
    }

    @Override
    public Map<String, Object> postProcessOperationsWithModels(Map<String, Object> objs, List<Object> allModels) {
        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allTheModels = new HashMap<String, CodegenModel>();

        for (Object obj : (List<Object>) allModels) {
            Map<String, Object> map = (Map<String, Object>) obj;

            CodegenModel cm = (CodegenModel) map.get("model");
            String opName = (String) patchOperationBodyNames.get(camelize(cm.getName()));
            if (opName != null) {
                cm.setClassname(toModelName(opName));
                cm.getVendorExtensions().put("x-rustgen-body-model", "true");
            }
            String resName = (String) patchOperationResponseNames.get(camelize(cm.getName()));
            if (resName != null) {
                cm.setClassname(toModelName(resName));
            }

            // Sanitize OneOf and AnyOf names, replace the /body[0-9]+/ naming with nicer
            // names stored previously.
            Pattern reBody = Pattern.compile("(^Body[0-9]+)");

            cm.classname = camelize(cm.classname.replaceFirst("OneOf", ""));
            cm.classname = camelize(cm.classname.replaceFirst("AnyOf", ""));
            Matcher matchBody = reBody.matcher(cm.classname);

            for (CodegenProperty property : cm.vars) {
                if (property.datatype.startsWith("OneOf")) {
                    property.datatype = camelize(property.datatype.replaceFirst("OneOf", ""));
                } else if (property.datatype.startsWith("AnyOf")) {
                    property.datatype = camelize(property.datatype.replaceFirst("AnyOf", ""));
                }

                if (property.getItems() != null && property.getItems().datatype.startsWith("OneOf")) {
                    property.getItems().datatype = camelize(property.getItems().datatype.replace("OneOf", ""));
                } else if (property.getItems() != null && property.getItems().datatype.startsWith("AnyOf")) {
                    property.getItems().datatype = camelize(property.getItems().datatype.replace("AnyOf", ""));
                }
            }

            if (matchBody.find()) {
                String body = matchBody.group(0);

                if (patchOperationBodyNames.containsKey(body)) {
                    cm.classname = cm.classname.replace(body, camelize((String) patchOperationBodyNames.get(body)))
                            + "Enum";
                }
            }

            for (CodegenProperty property : cm.vars) {

                Matcher matchProp = reBody.matcher(property.datatype);
                if (matchProp.find()) {
                    property.datatype = property.datatype.replace(matchProp.group(0),
                            camelize((String) patchOperationBodyNames.get(matchProp.group(0)))) + "Enum";
                }

                if (property.getItems() != null) {
                    matchProp = reBody.matcher(property.getItems().datatype);

                    if (matchProp.find()) {
                        property.getItems().datatype = property.getItems().datatype.replace(matchProp.group(0),
                                camelize((String) patchOperationBodyNames.get(matchProp.group(0)))) + "Enum";
                    }
                }
            }

            if (!allTheModels.containsKey(cm.classname)) {
                allTheModels.put(cm.classname, cm);
            }
        }

        for (Object obj : objs.values()) {
            if (obj instanceof Map) {
                Map<String, Object> map = (Map<String, Object>) obj;

                List<CodegenOperation> ops = (List<CodegenOperation>) map.get("operation");

                for (CodegenOperation operation : ops) {
                    if (operation.operationId.startsWith("list_")) {

                        List<CodegenResponse> responses = operation.getResponses();
                        for (final CodegenResponse res : responses) {
                            if (res.getDataType() != null) {
                                if (getBooleanValue(res, CodegenConstants.IS_DEFAULT_EXT_NAME)) {
                                    CodegenModel mdl = allTheModels.get(res.dataType);

                                    if (mdl != null) {
                                        if (mdl.getVendorExtensions().get("x-codegen-pagination-response-inner") != null) {
                                            res.getVendorExtensions().put("x-codegen-pagination-response-inner", mdl.getVendorExtensions().get("x-codegen-pagination-response-inner"));
                                            operation.getVendorExtensions().put("x-codegen-is-list-fn", "true");
                                            operation.getVendorExtensions().put("x-codegen-response-plural", res.dataType.replace("Response", "ListResponse"));
                                            operation.getVendorExtensions().put("x-codegen-response-plural-snake-case", underscore(res.dataType.replace("Response", "")));
                                            operation.getVendorExtensions().put("x-codegen-response-single", mdl.getVendorExtensions().get("x-codegen-pagination-response-inner"));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return objs;
    }

    @Override
    public Map<String, Object> postProcessOperations(Map<String, Object> objs) {
        objs = super.postProcessOperations(objs);
        Map<String, Object> operations = (Map<String, Object>) objs.get("operations");
        if (operations != null) {
            //LOGGER.info(" *** " + operations.keySet() );
            //operations.getVendorExtensions().put("x-rustgen-plural-snake-case", "fooofooo");

            List<CodegenOperation> ops = (List<CodegenOperation>) operations.get("operation");
            for (final CodegenOperation operation : ops) {
                if (operation.notes != null) {
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("```(.+)\\n(.+)\\n",
                            "```$1,nocompile\n$2\n");
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("```\\n(.+)\\n",
                            "```nocompile\n$1\n");
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("\\n", "\n    /// ");

                }

                CodegenParameter body = operation.bodyParam;
                if (body != null) {
                    String opName = (String) patchOperationBodyNames.get(camelize(body.getDataType()));
                    if (opName != null) {
                        body.dataType = toModelName(opName);
                    }
                }
                List<CodegenResponse> responses = operation.getResponses();
                Boolean hasDefaultResponse = false;
                for (final CodegenResponse res : responses) {
                    if (res.getDataType() != null) {
                        if (getBooleanValue(res, CodegenConstants.IS_DEFAULT_EXT_NAME)) {
                            hasDefaultResponse = true;
                        }
                        String resName = (String) patchOperationResponseNames.get(camelize(res.getDataType()));
                        if (resName != null) {
                            res.dataType = toModelName(resName);
                        }
                        //operation.getVendorExtensions().put("x-codegen-response-plural", res.dataType.replace("Response", "ListResponse"));
                    }
                }
                if (!hasDefaultResponse) {
                    operation.getVendorExtensions().put("x-codegen-response-empty-default", "true");
                }
                List<CodegenParameter> queryParams = operation.queryParams;
                Boolean hasOptionalQueryParams = true;
                Boolean hasStringParams = false;
                for (final CodegenParameter param : queryParams) {
                    if (getBooleanValue(param, CodegenConstants.IS_LIST_CONTAINER_EXT_NAME)
                            && getBooleanValue(param.items, CodegenConstants.IS_STRING_EXT_NAME)) {
                        param.getVendorExtensions().put("x-codegen-list-container-string", "true");
                    }
                    if (param.paramName.equals("total")) {
                        param.getVendorExtensions().put("x-codegen-ignore", "true");
                    }
                    if (param.getRequired()) {
                        hasOptionalQueryParams = false;
                    }
                    if (getBooleanValue(param, CodegenConstants.IS_STRING_EXT_NAME)
                            || getBooleanValue(param, CodegenConstants.IS_UUID_EXT_NAME)) {
                        hasStringParams = true;
                    }
                }
                if (hasOptionalQueryParams) {
                    operation.getVendorExtensions().put("x-codegen-has-optional-query-params", "true");
                }
                if (hasStringParams) {
                    operation.getVendorExtensions().put("x-codegen-has-string-params", "true");
                }

            }
        }
        return objs;
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);

        if (property.datatype.equals("isize")) {
            // needed for windows
            property.datatype = "i64";
        }

    }

    @Override
    public String toEnumVarName(String value, String datatype) {
        String name = super.toEnumVarName(value, datatype);
        if (name.length() == 0) {
            return "EMPTY";
        }
        return name;
    }

    @Override
    public CodegenParameter fromRequestBody(RequestBody body, String name, Schema schema, Map<String, Schema> schemas,
            Set<String> imports) {
        CodegenParameter param = super.fromRequestBody(body, name, schema, schemas, imports);

        // patches the Body* Models with better names
        if (body.getExtensions() != null) {
            Object operationName = body.getExtensions().get("x-codegen-operation-name");
            if (operationName != null && !operationName.toString().isEmpty() && name != null) {
                LOGGER.info(" *** " + removeVerb(operationName.toString()));
                //patchOperationBodyNames.put(camelize(name), removeVerb(operationName.toString() + "Body"));
                patchOperationBodyNames.put(camelize(name), operationName + "Body");

            }
        }

        // Needed because of the static call to `::from_json`, which requires an
        // unqualified type
        if (schema instanceof ObjectSchema && param.getDataType().startsWith("HashMap")) {
            param.getVendorExtensions().put(CodegenConstants.IS_MAP_CONTAINER_EXT_NAME, Boolean.TRUE);
            param.getVendorExtensions().put(CodegenConstants.IS_CONTAINER_EXT_NAME, Boolean.TRUE);
        }

        return param;
    }

    @Override
    public void addParentContainer(CodegenModel codegenModel, String name, Schema schema) {
        super.addParentContainer(codegenModel, name, schema);
    }

    @Override
    public CodegenResponse fromResponse(String responseCode, ApiResponse response) {
        CodegenResponse res = super.fromResponse(responseCode, response);

        if (response.getExtensions() != null) {
            Object operationName = response.getExtensions().get("x-codegen-operation-name");
            if (operationName != null && !operationName.toString().isEmpty() && res.getDataType() != null
                    && res.getDataType().startsWith("InlineResponse") && responseCode.equals("200")) {
                patchOperationResponseNames.put(camelize(res.getDataType()),
                        operationName.toString() + "Response");
            }
        }

        // Special case
        if (res.getDataType() != null && res.getDataType().equals("SelectedActions")) {
            res.dataType = "PutActionsSetAllowedActionsRepository";
        }

        return res;
    }

    String removeVerb(String opName) {
        if (opName.startsWith("Update")) {
            return opName.replace("Update", "");
        } else if (opName.startsWith("List")) {
            return opName.replace("List", "");
        } else if (opName.startsWith("Create")) {
            return opName.replace("Create", "");
        } else if (opName.startsWith("Merge")) {
            return opName.replace("Merge", "");
        } else {
            return opName;
        }
    }

    @Override
    public String getSchemaType(Schema property) {
        String schemaType = super.getSchemaType(property);
        if (schemaType != null) {
            return schemaType.replace("UBUNTU", "ubuntu").replace("MACOS", "macos").replace("WINDOWS", "windows");
        } else {
            return null;
        }
    }

    @Override
    public String toOperationId(String operationId) {
        operationId = operationId.replaceFirst("[a-zA-Z0-9]+\\/", "");

        return super.toOperationId(operationId);
    }

    @Override
    public void addHandlebarHelpers(Handlebars handlebars) {
        super.addHandlebarHelpers(handlebars);
        handlebars.registerHelpers(new IfCondHelper());
    }

    @Override
    public String toVarName(String name) {
        if (name.equals("ref")) {
            return "git_ref";
        }

        return super.toVarName(name);
    }
}
