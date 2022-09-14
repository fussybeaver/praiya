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
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.responses.ApiResponse;
import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
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

    private static final HashMap<String, Object> patchOperationBodyNames = new HashMap();

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

                    // For types that indicate they inherit the `Reference`
                    // type, we manually add properties from the `Tag` Schema,
                    // because of limitations in copying polymorphic models the
                    // swagger code generator.
                    if (type.equals("Tag/allOf/0") || type.equals("Reference") || mdl.name.endsWith("ContactMethod")) {
                        Schema refSchema = null;
                        String ref = io.swagger.codegen.v3.generators.util.OpenAPIUtil.getSimpleRef("Tag");

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

        if (propertySchema instanceof ComposedSchema) {
            ComposedSchema composedSchema = (ComposedSchema) propertySchema;
            if (composedSchema.getOneOf() != null) {
                List<Schema> schemas;
                schemas = composedSchema.getOneOf();
                HashMap<String, Object> allowableValues = new HashMap();
                ArrayList<String> values = new ArrayList();
                ArrayList<HashMap<String, String>> enumVars = new ArrayList();
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
                    }
                }
                allowableValues.put("values", values);
                allowableValues.put("untaggedVars", enumVars);
                prop.allowableValues = allowableValues;
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
                allModels.put(modelName, cm);
            }
        }

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
            }

            // We remove all inline generated types, as these are now encoded
            // into proc-macros and associated with individual endpoints. So,
            // for example, a response type for a 'get incident' API will have
            // an associated macro generated response struct.
            if (model.name.startsWith("inline_response")) {
                model.vendorExtensions.put("x-rustgen-noop", true);
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
                if (prop.datatype != null && !(prop.datatype.startsWith("chrono"))) {
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
