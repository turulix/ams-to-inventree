# PatchedUserProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | Option<**String**> | Preferred language for the user | [optional]
**theme** | Option<**serde_json::Value**> | Settings for the web UI as JSON - do not edit manually! | [optional]
**widgets** | Option<**serde_json::Value**> | Settings for the dashboard widgets as JSON - do not edit manually! | [optional]
**displayname** | Option<**String**> | Chosen display name for the user | [optional]
**position** | Option<**String**> | Main job title or position | [optional]
**status** | Option<**String**> | User status message | [optional]
**location** | Option<**String**> | User location information | [optional]
**active** | Option<**bool**> | User is actively using the system | [optional]
**contact** | Option<**String**> | Preferred contact information for the user | [optional]
**r#type** | Option<[**models::UserTypeEnum**](UserTypeEnum.md)> | Which type of user is this?  * `bot` - Bot * `internal` - Internal * `external` - External * `guest` - Guest | [optional]
**organisation** | Option<**String**> | Users primary organisation/affiliation | [optional]
**primary_group** | Option<**i32**> | Primary group for the user | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


