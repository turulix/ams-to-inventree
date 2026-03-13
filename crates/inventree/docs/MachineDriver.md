# MachineDriver

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slug** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**provider_file** | **String** | File that contains the class definition. | [readonly]
**provider_plugin** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Plugin(s) that contain(s) the class definition. | [optional][readonly]
**is_builtin** | **bool** | Indicates if the machine type is build into the InvenTree source code. | [readonly]
**machine_type** | **String** |  | [readonly]
**driver_errors** | **Vec<String>** | Errors registered against driver. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


