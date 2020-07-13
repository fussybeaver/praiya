package praiya;

import io.swagger.models.parameters.Parameter;
import io.swagger.models.parameters.SerializableParameter;
import io.swagger.models.parameters.BodyParameter;
import io.swagger.util.Json;

import io.swagger.codegen.v3.CliOption;
import io.swagger.codegen.v3.CodegenArgument;
import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.generators.DefaultCodegenConfig;
import io.swagger.codegen.v3.generators.handlebars.java.JavaHelper;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.PathItem;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.responses.ApiResponse;
import io.swagger.v3.parser.util.SchemaTypeUtil;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.apache.commons.lang3.StringUtils;

import java.util.*;
import java.util.Map.Entry;
import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

public class PraiyaCodegen extends RustServerCodegen {
  private static final Logger LOGGER = LoggerFactory.getLogger(PraiyaCodegen.class);

  public PraiyaCodegen() {
    super();
  }

  // Declare custom additions to inline enums that are behaving differently
  // than the official spec
  // private static HashMap<String, List<Map<String, String>>> patchEnumValues;
  // static {
  // patchEnumValues = new HashMap<String, List<Map<String, String>>>();
  // Map<String, String> additionalEnumValues = new HashMap<String, String>();
  // List<Map<String, String>> enumValues = new ArrayList <Map<String, String>>();

  // additionalEnumValues.put("name", "ROLLBACK_UPDATING");
  // additionalEnumValues.put("value", "\"rollback_updating\"");
  // enumValues.add(additionalEnumValues);

  // additionalEnumValues = new HashMap<String, String>();
  // additionalEnumValues.put("name", "ROLLBACK_PAUSED");
  // additionalEnumValues.put("value", "\"rollback_paused\"");
  // enumValues.add(additionalEnumValues);

  // additionalEnumValues = new HashMap<String, String>();
  // additionalEnumValues.put("name", "ROLLBACK_COMPLETED");
  // additionalEnumValues.put("value", "\"rollback_completed\"");
  // enumValues.add(additionalEnumValues);

  // patchEnumValues.put("ServiceUpdateStatusStateEnum", enumValues);
  // }

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

    // This is a "fallback" type, and allows some parts of the Docker API
    // that receive an empty JSON '{}' value.
    if ("object".equals(type)) {
      type = "HashMap<(), ()>";
    }

    return type;
  }

  @Override
  public CodegenProperty fromProperty(String name, Schema p) {
    CodegenProperty property = super.fromProperty(name, p);

    // Remove extraneous references
    if (property.datatype.startsWith("models::")) {
      property.datatype = property.datatype.replace("models::", "");
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
      }
    }

    for (Entry<String, CodegenModel> entry : allModels.entrySet()) {
      CodegenModel model = entry.getValue();
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

          // add additional enum values that get patched in at the template level
          // if (patchEnumValues.containsKey(model.classname + prop.enumName)) {
          // prop.vendorExtensions.put("x-rustgen-additional-enum-values",
          // patchEnumValues.get(model.classname + prop.enumName));
          // }
        }
      }
    }

    return newObjs;
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
}
