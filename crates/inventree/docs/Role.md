# Role

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | **i32** |  | 
**username** | **String** | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. | 
**roles** | **std::collections::HashMap<String, serde_json::Value>** | Roles associated with the user. | [readonly]
**permissions** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Permissions associated with the user. | [optional][readonly]
**is_staff** | Option<**bool**> | Designates whether the user can log into this admin site. | [optional]
**is_superuser** | Option<**bool**> | Designates that this user has all permissions without explicitly assigning them. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


