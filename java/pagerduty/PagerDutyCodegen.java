package pagerduty;

import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import io.swagger.codegen.v3.*;
import io.swagger.codegen.v3.generators.util.OpenAPIUtil;
import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ComposedSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.core.util.Json;

public class PagerDutyCodegen extends RustServerCodegen {
    private static final Logger LOGGER = LoggerFactory.getLogger(PagerDutyCodegen.class);

    public PagerDutyCodegen() {
        super();

        // This client does not generate Rust API endpoints from java, it just generates models.
        supportingFiles.remove(new SupportingFile("endpoints.mustache", "src/endpoints", "mod.rs"));
        apiTemplateFiles.remove("api.mustache");
        cliOptions.add(CliOption.newString("targetApiPrefix", "target model prefix"));
        supportingFiles.remove(new SupportingFile("models.mustache", "src", "models.rs"));

        cliOptions.add(CliOption.newString("skipSerializingIfNone", "whether to serialize None as fields with null javascript values"));

    }

    @Override
    public void processOpts() {
        super.processOpts();

        // This overrides the target path for the models being generated,
        // which is useful when the upstream vendor has multiple openapi
        // schemas for different parts of the API.
        if (additionalProperties.get("targetApiPrefix") != null) {
            supportingFiles.add(new SupportingFile("models.mustache", "src", String.format("%s_models.rs", additionalProperties.get("targetApiPrefix"))));
        }
    }

    private static final Map<String, Object> patchOperationBodyNames = new HashMap();
    private static final Map<String, List<String>> referenceOverrideExceptions = new HashMap();
    static {
        referenceOverrideExceptions.put("Incident", Arrays.asList("LogEntry"));
        referenceOverrideExceptions.put("Team", Arrays.asList("Team"));
        referenceOverrideExceptions.put("ResolveReason", Arrays.asList("Incident"));
        // Note: this is really MergeIncidents
        referenceOverrideExceptions.put("IdMergeBody", Arrays.asList("Incident"));
    }

    @Override
    public void preprocessOpenAPI(OpenAPI openAPI) {
        Info info = openAPI.getInfo();
        List versionComponents = new ArrayList(Arrays.asList(info.getVersion().split("[.]")));
        while (versionComponents.size() < 3) {
            // Add the package version as a version component to the official specification
            // version
            versionComponents.add(additionalProperties.get(CodegenConstants.PACKAGE_VERSION));
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
        //List excludedModels = ArrayList<String>("AllOfOrchestrationUnrouted_orchestration_path_rulesActions");

        // This model schema crashes the generator - do not remove
        // We generate a dummy model and return before calling the super method
        if (name.equals("inline_response_200_11") || name.equals("AllOfOrchestrationUnrouted_orchestration_path_rulesActions") || name.equals("AllOfServiceOrchestration_orchestration_path_catch_allActions") || name.equals("AllOfServiceOrchestration_orchestration_path_rulesActions")) {
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

        // Partially deal with inline object polymorphism: 'anyOf' and 'oneOf'.
        if (schema instanceof ComposedSchema) {
            ComposedSchema composedSchema = (ComposedSchema) schema;
            if (composedSchema.getAllOf() != null) {
                List<Schema> schemas;
                schemas = composedSchema.getAllOf();
                for (Schema subSchema : schemas) {
                    String type = getTypeDeclaration(subSchema);

                    // Address swagger generator's limitation in generating
                    // polymorphic models with a deep reference into a composed
                    // openaPI schema.
                    //
                    // For some specific types we manually add properties from
                    // the `Tag` Schema. In other cases, we infer the parent
                    // schema and copy properties from there.
                    //
                    if (type.endsWith("/allOf/0") || type.equals("Reference") || mdl.name.endsWith("ContactMethod")) {
                        Schema refSchema = null;
                        String ref = null;
                        if (type.endsWith("/allOf/0")) {
                            // Copy properties from the parent model of a composed schema deep link
                            String prefix = type.substring(0, type.length() - 8);
                            String parentModel;
                            // In some cases we have an absolute reference, in others we don't
                            if (prefix.indexOf("/") >= 0) {
                                parentModel = prefix.substring(0, prefix.lastIndexOf("/"));
                            } else {
                                parentModel = prefix;
                            }
                            ref = io.swagger.codegen.v3.generators.util.OpenAPIUtil.getSimpleRef(parentModel);

                        } else {
                            ref = io.swagger.codegen.v3.generators.util.OpenAPIUtil.getSimpleRef("Tag");
                        }

                        // Some fields on a `Reference` are required and can be
                        // auto-filled. These are special-cased in the handlebars template.
                        List<Map<String, String>> defaultImpl = new ArrayList();
                        Map<String, String> labelImpl = new HashMap();
                        labelImpl.put("key", "label");
                        labelImpl.put("value", name);
                        defaultImpl.add(labelImpl);
                        Map<String, String> typeImpl = new HashMap();
                        typeImpl.put("key", "_type");
                        typeImpl.put("value", underscore(name));
                        defaultImpl.add(typeImpl);

                        // Marking the model with a default impl will trigger a non-derived Default
                        mdl.getVendorExtensions().put("x-rustgen-default-impl", defaultImpl);
                        mdl.getVendorExtensions().put("x-rustgen-has-default-impl", true);

                        if (allDefinitions != null) {
                            refSchema = allDefinitions.get(ref);
                            if (refSchema instanceof ComposedSchema) {
                                final ComposedSchema refComposed = (ComposedSchema) refSchema;
                                final List<Schema> allOf = refComposed.getAllOf();
                                if (allOf != null && !allOf.isEmpty()) {
                                    for (Schema interfaceSchema : allOf) {
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

                                                    // Copy the Tag model properties into the current model
                                                    mdl.vars.add(0, codegenProperty);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        for (CodegenProperty codegenProperty : mdl.vars) {
                            if (codegenProperty.name.equals("label") || codegenProperty.name.equals("_type")) {
                                codegenProperty.getVendorExtensions().put("x-rustgen-is-required", true);

                                // Marking the property with a default impl
                                // ensures that properties like `type` are set
                                // appropriately in a non-derive Default
                                // implementation
                                if (codegenProperty.name.equals("_type")) {
                                    codegenProperty.getVendorExtensions().put("x-rustgen-default-impl", underscore(mdl.name));
                                } else {
                                    codegenProperty.getVendorExtensions().put("x-rustgen-default-impl", mdl.name);
                                }
                                codegenProperty.getVendorExtensions().put("x-rustgen-has-default-impl", true);
                            }
                        }
                    }

                    // Tag pagination models with their inner data type.
                    if (type.equals("Pagination")) {
                        for (CodegenProperty item : mdl.vars) {
                            if (item.containerType != null && item.containerType.equals("array")) {
                                mdl.getVendorExtensions().put("x-codegen-pagination-response-inner", item.items.datatype);
                            }
                        }
                    }

                }
            }
        }

        return mdl;
    }

    @Override
    public CodegenProperty fromProperty(String name, Schema propertySchema) {
        CodegenProperty prop = super.fromProperty(name, propertySchema);

        // Address models that seem to be missing in the swagger generator
        // 'fromModel' pass. These are mostly composed schemas. We check properties,
        // fill appropriate vendor extensions and deal with these exceptions at
        // the template level.
        //
        // In some cases, these are enums, of which we handle in three ways
        // (only the latter two are handled in this part of the code):
        //   - An enumeration of constant strings;
        //   - An enumeration of models, if the swagger generator can handle
        //   the reference;
        //   - An enumeration of anonymous structs, for openAPI references that
        //   the swagger generator doesn't handle.
        //
        // In other cases, we generate an inline model and pass it in vendor
        // extensions. 
        if (propertySchema instanceof ComposedSchema) {
            ComposedSchema composedSchema = (ComposedSchema) propertySchema;
            if (composedSchema.getOneOf() != null || composedSchema.getAnyOf() != null) {
                List<Schema> schemas;
                if (composedSchema.getAnyOf() != null) {
                    schemas = composedSchema.getAnyOf();
                } else {
                    schemas = composedSchema.getOneOf();
                }
                HashMap<String, Object> allowableValues = new HashMap();
                ArrayList<String> values = new ArrayList();
                ArrayList<HashMap<String, String>> enumVars = new ArrayList();
                ArrayList<HashMap<String, CodegenModel>> enumComplex = new ArrayList();

                prop.enumName = "Enum";
                for (Schema subSchema : schemas) {
                    if (subSchema.get$ref() != null) {
                        String ref = OpenAPIUtil.getSimpleRef(subSchema.get$ref());
                        values.add(ref);
                        HashMap<String, String> enumVarsProp = new HashMap();
                        enumVarsProp.put("name", StringUtils.upperCase(underscore(ref)));
                        enumVarsProp.put("value", ref);
                        enumVars.add(enumVarsProp);
                        prop.getVendorExtensions().put("is-enum", true);
                        prop.getVendorExtensions().put("x-rustgen-is-untagged-enum", true);
                        prop.enumName = camelize(name) + "Enum";
                    } else {
                        if (subSchema.getProperties() != null) {
                            CodegenModel mdl = fromModel(name, subSchema);
                            HashMap<String, CodegenModel> enumVarsProp = new HashMap();
                            enumVarsProp.put("value", mdl);
                            enumComplex.add(enumVarsProp);
                            prop.getVendorExtensions().put("is-enum", true);
                            prop.getVendorExtensions().put("x-rustgen-is-untagged-enum", true);
                            prop.getVendorExtensions().put("x-rustgen-is-complex-enum", true);
                            prop.enumName = camelize(name) + "Items";
                        }
                    }
                }
                allowableValues.put("values", values);
                allowableValues.put("untaggedVars", enumVars);
                allowableValues.put("complexVars", enumComplex);
                prop.allowableValues = allowableValues;
            } else if (composedSchema.getAllOf() != null) {
                List<Schema> schemas;
                schemas = composedSchema.getAllOf();

                final CodegenModel codegenModel = CodegenModelFactory.newInstance(CodegenModelType.MODEL);
                if (reservedWords.contains(name)) {
                    codegenModel.name = escapeReservedWord(name);
                } else {
                    codegenModel.name = name;
                }
                codegenModel.classname = toModelName(name);
                codegenModel.classVarName = toVarName(name);
                codegenModel.classFilename = toModelFilename(name);
                List<String> openApiRefs = new ArrayList();
                for (Schema subSchema : schemas) {
                    if (subSchema.get$ref() != null) {
                        String ref = OpenAPIUtil.getSimpleRef(subSchema.get$ref());
                        if (!ref.equals(camelize(ref)) || ref.matches("^[0-9]*$")) {
                            // Handle deep references that the swagger
                            // generator cannot link. e.g.
                            // #/components/schemas/OrchestrationUnrouted/allOf/1/properties/orchestration_path/properties/catch_all/properties/actions
                            if (subSchema.get$ref().contains("OrchestrationUnrouted")) {
                                openApiRefs.add("OrchestrationUnroutedOrchestrationPathCatchAllActions");
                            } else if (subSchema.get$ref().contains("ServiceOrchestration")) {
                                openApiRefs.add("ServiceOrchestrationOrchestrationPathCatchAllActions");
                            }
                        } else {
                            openApiRefs.add(ref);
                        }
                    } else {
                        CodegenModel subMdl = fromModel("tmp", subSchema);
                        for (CodegenProperty subProp : subMdl.vars) {
                            codegenModel.vars.add(subProp);
                        }
                    }
                }
                codegenModel.getVendorExtensions().put("x-rustgen-additional-var-refs", openApiRefs);
                prop.getVendorExtensions().put("x-rustgen-additional-model", codegenModel);
            }
        }

        return prop;
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
                allModels.put(cm.classname, cm);
            }
        }

        List<String> inlineResponses = new ArrayList();
        for (Entry<String, CodegenModel> entry : allModels.entrySet()) {
            CodegenModel model = entry.getValue();

            if (model.getDataType() != null && model.getDataType().equals("object")) {
                model.setDataType(entry.getKey());
            }
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
            } else if (model.vars.isEmpty() && !model.getIsArrayModel()) {
                // Do not generate 'empty' structs.
                //
                // Generally, these will be structs that the swagger generator
                // decided to include but failed to populate due to the model
                // being a composed schema.
                //
                // We now handle these by generating them through property
                // vendor extensions and templating.
                model.vendorExtensions.put("x-rustgen-noop", true);
            }

            // We remove all inline generated types, as these are now encoded
            // into proc-macros and associated with individual endpoints. So,
            // for example, a response type for a 'get incident' API will have
            // an associated macro generated response struct.
            if (model.classname.startsWith("InlineResponse")) {
                if (model.classname.replace("InlineResponse", "").matches("^[0-9]*$")) {
                    model.vendorExtensions.put("x-rustgen-noop", true);
                } else {
                    // We do want to generate nested Inline Response schemas,
                    // where they are missing.
                    String classname = model.classname.replaceFirst("^InlineResponse[0-9]*", "");
                    if (!inlineResponses.contains(classname) && !allModels.keySet().contains(classname)) {
                        model.classname = classname;
                        inlineResponses.add(classname);
                    } else {
                        model.vendorExtensions.put("x-rustgen-noop", true);
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

                if (prop.baseName.equals("score") && prop.datatype.equals("i64")) {
                    prop.datatype = "f32";
                }

                if (prop.datatype != null && prop.datatype.contains("_")) {
                    prop.datatype = camelize(prop.datatype);
                }

                // Skip properties that exist but are empty strings
                if (prop.baseName != null && prop.baseName.isEmpty()) {
                    prop.vendorExtensions.put("x-rustgen-skip-prop", true);
                }

                // All PagerDuty `type` properties are Strings
                if (prop.baseName != null && prop.baseName.equals("type") && prop.datatype.equals("Value")) {
                    prop.datatype = "String";
                }

                if (prop.baseName != null && !prop.baseName.equals("type") && prop.allowableValues != null && prop.datatype.equals("String")) {
                    ArrayList<HashMap<String, String>> vars = (ArrayList<HashMap<String, String>>) prop.allowableValues.get("enumVars");
                    if (vars != null && vars.size() > 0) {
                        prop.vendorExtensions.put("is-enum", true);
                    }
                }

                // For array types containing enums, where the swagger
                // generator failed to find a valid datatype, we set the type
                // to a standardised `classname`+`propertyname`+`Enum` 
                if (prop.getItems() != null && prop.getItems().getVendorExtensions() != null && getBooleanValue(prop.getItems(), "is-enum") && prop.getItems().datatype.equals("Value")) {
                    prop.getItems().datatype = camelize(model.classname) + prop.getItems().enumName;
                }

                // Sanitise some datatypes, remove the composed prefix and
                // inline response from property datatypes, since we generate
                // these now with better names.

                if (prop.datatype.startsWith("AllOf")) {
                    if (!allModels.keySet().contains(prop.datatype) || allModels.get(prop.datatype).vars.isEmpty()) {
                        prop.datatype = prop.datatype.replace("AllOf", "");
                    }
                }

                if (prop.datatype.startsWith("InlineResponse")) {
                    prop.datatype = prop.datatype.replaceFirst("^InlineResponse[0-9]*", "");
                }

                if (prop.getItems() != null && prop.getItems().datatype.startsWith("InlineResponse")) {
                    prop.getItems().datatype = prop.getItems().datatype.replaceFirst("^InlineResponse[0-9]*", "");
                }

                // Don't use Reference types, if possible, just point to the full model.
                // This ensures that we can use 'include' parameters in API endpoints.
                // We make sure required fields are marked optional on the full model to ensure we can deserialize.

                if (prop.datatype.endsWith("Reference")) {
                    String fullMdlName = prop.datatype.replaceFirst("Reference$", "");

                    if (allModels.keySet().contains(fullMdlName)) {
                        if (!referenceOverrideExceptions.keySet().contains(model.classname)
                                || !referenceOverrideExceptions.get(model.classname).contains(fullMdlName)) {
                            prop.datatype = fullMdlName;
                        }
                    }
                }

                if (prop.getItems() != null && prop.getItems().datatype.endsWith("Reference")) {
                    String fullMdlName = prop.getItems().datatype.replaceFirst("Reference$", "");

                    if (allModels.keySet().contains(fullMdlName)) {
                        if (!referenceOverrideExceptions.keySet().contains(model.classname)
                                || !referenceOverrideExceptions.get(model.classname).contains(fullMdlName)) {
                            prop.getItems().datatype = fullMdlName;
                        }
                    }
                }

                if (prop.datatype != null && prop.datatype.equals("String") && prop.allowableValues == null) {
                    prop.vendorExtensions.put("x-rustgen-is-string", true);
                    model.vendorExtensions.put("x-rustgen-has-string", true);
                }

                // Many PagerDuty models have a `label` or `type` field that
                // are required when creating or updating the API. We leverage
                // Rust's `Default` trait to fill these.
                //
                // This codepath is usually triggered by PagerDuty models that
                // reference the `Tag` model. Swagger code generator is able to
                // merge these properties, unlike with models referencing the
                // `Reference` model, so we need a second check on `label` and
                // `type` properties to ensure they can both generate default
                // values and have serde generate values if they are missing.

                if (prop.baseName != null && (prop.baseName.equals("type") || prop.baseName.equals("label"))) {
                    prop.vendorExtensions.put("x-rustgen-is-required", true);
                    List<Map<String, String>> defaultImpl = (List<Map<String, String>>) model.getVendorExtensions().get("x-rustgen-default-impl");
                    if (defaultImpl == null) {
                        defaultImpl = new ArrayList();
                    }

                    Map<String, String> typeImpl = new HashMap();
                    if (prop.baseName.equals("type")) {
                        typeImpl.put("key", "_type");
                    } else if (prop.baseName.equals("label")) {
                        typeImpl.put("key", "label");
                    }
                    String typeValue = null;
                    if (getBooleanValue(prop, CodegenConstants.IS_ENUM_EXT_NAME)) {
                        ArrayList<HashMap<String, String>> vars = (ArrayList<HashMap<String, String>>) prop.allowableValues
                                .get("enumVars");
                        if (vars.size() > 0) {
                            prop.vendorExtensions.put("is-enum", true);
                        }

                    } else {
                        if (prop.baseName.equals("type")) {
                            typeImpl.put("value", underscore(model.name));
                            prop.getVendorExtensions().put("x-rustgen-default-impl", underscore(model.name));
                        } else if (prop.baseName.equals("label")) {
                            typeImpl.put("value", model.name);
                            prop.getVendorExtensions().put("x-rustgen-default-impl", model.name);
                        }
                        if (!defaultImpl.contains(typeImpl)) {
                            defaultImpl.add(typeImpl);
                        }
                        model.getVendorExtensions().put("x-rustgen-default-impl", defaultImpl);
                        model.getVendorExtensions().put("x-rustgen-has-default-impl", true);
                        prop.getVendorExtensions().put("x-rustgen-has-default-impl", true);
                    }
                }

                // Do not pass the skip_serializing_if_none serde proc macro if
                // property's value is None.
                if (additionalProperties.get("skipSerializingIfNone").equals("false")) {
                    prop.vendorExtensions.put("x-rustgen-no-skip-none", true);
                }
            }

            // Required properties should not be wrapped in an Option.
            //
            // This codepath has quite a big effect on ergonomics in the
            // library, as the swagger `required` block is honoured and values
            // are no longer wrapped in the `Option` type. That does incur the
            // risk though that the upstream API conforms closely to these
            // required fields, or we will crash on deserialization.
            for (CodegenProperty prop : model.requiredVars) {
                // chrono does not implement Default
                if (prop.datatype != null && !(prop.datatype.startsWith("chrono")) && !allModels.containsKey(model.classname + "Reference")) {
                    prop.vendorExtensions.put("x-rustgen-is-required", true);
                }
            }
        }

        return newObjs;
    }

    @Override
    public Map<String, Object> postProcessOperationsWithModels(Map<String, Object> objs, List<Object> allModels) {
        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allTheModels = new HashMap<String, CodegenModel>();

        for (Object obj : allModels) {
            Map<String, Object> map = (Map<String, Object>) obj;

            CodegenModel cm = (CodegenModel) map.get("model");

            // Replace names of PUT/POST operation body parameter model names,
            // with nicer generated ones.
            String opName = (String) patchOperationBodyNames.get(camelize(cm.getName()));
            if (opName != null) {
                cm.setClassname(toModelName(opName));
                cm.getVendorExtensions().put("x-rustgen-body-model", "true");
            }

            // Sanitize OneOf and AnyOf names, replace the /body[0-9]+/ naming with nicer
            // names stored previously.
            Pattern reBody = Pattern.compile("(^Body[0-9]+)");

            cm.classname = camelize(cm.classname.replaceFirst("OneOf", ""));
            cm.classname = camelize(cm.classname.replaceFirst("AnyOf", ""));
            Matcher matchBody = reBody.matcher(cm.classname);

            // Replace the OneOf/AnyOf prefix to a model
            for (CodegenProperty property : cm.vars) {
                if (property.datatype.startsWith("OneOf")) {
                    property.datatype = camelize(property.datatype.replaceFirst("OneOf", ""));
                } else if (property.datatype.startsWith("AnyOf")) {
                    property.datatype = camelize(property.datatype.replaceFirst("AnyOf", ""));
                }

                // Replace the child type if this is type is a map/array
                if (property.getItems() != null && property.getItems().datatype.startsWith("OneOf")) {
                    property.getItems().datatype = camelize(property.getItems().datatype.replace("OneOf", ""));
                } else if (property.getItems() != null && property.getItems().datatype.startsWith("AnyOf")) {
                    property.getItems().datatype = camelize(property.getItems().datatype.replace("AnyOf", ""));
                }
            }

            if (!allTheModels.containsKey(cm.classname)) {
                allTheModels.put(cm.classname, cm);
            }
        }
        return objs;
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);

        if (property.datatype.equals("Override")) {
            property.datatype = "ModelOverride";
        }
    }

    @Override
    public CodegenParameter fromRequestBody(RequestBody body, String name, Schema schema, Map<String, Schema> schemas,
                                            Set<String> imports) {
        CodegenParameter param = super.fromRequestBody(body, name, schema, schemas, imports);

        // patches the Body* Models with better names
        if (body.getExtensions() != null) {
            Object operationName = body.getExtensions().get("x-codegen-operation-name");
            if (operationName != null && !operationName.toString().isEmpty() && name != null) {
                patchOperationBodyNames.put(camelize(name), operationName);
            }
        }

        return param;
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
}
