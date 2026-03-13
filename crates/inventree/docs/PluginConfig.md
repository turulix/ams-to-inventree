# PluginConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**key** | **String** | Key of plugin | [readonly]
**name** | Option<**String**> | Name of the plugin | [optional]
**package_name** | Option<**String**> | Name of the installed package, if the plugin was installed via PIP | [optional]
**active** | Option<**bool**> | Is the plugin active | [optional]
**meta** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]
**mixins** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]
**is_builtin** | **bool** | Return True if this is a 'builtin' plugin. | [readonly]
**is_sample** | **bool** | Is this plugin a sample app? | [readonly]
**is_installed** | **bool** | Simple check to determine if this plugin is installed.  A plugin might not be installed if it has been removed from the system, but the PluginConfig associated with it still exists. | [readonly]
**is_package** | **bool** | Return True if this is a 'package' plugin. | [readonly]
**is_mandatory** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


