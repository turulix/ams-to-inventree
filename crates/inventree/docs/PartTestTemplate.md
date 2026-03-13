# PartTestTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**key** | **String** |  | [readonly]
**part** | **i32** |  | 
**test_name** | **String** | Enter a name for the test | 
**description** | Option<**String**> | Enter description for this test | [optional]
**enabled** | Option<**bool**> | Is this test enabled? | [optional]
**required** | Option<**bool**> | Is this test required to pass? | [optional]
**requires_value** | Option<**bool**> | Does this test require a value when adding a test result? | [optional]
**requires_attachment** | Option<**bool**> | Does this test require a file attachment when adding a test result? | [optional]
**results** | **i32** | Number of results recorded against this template | [readonly]
**choices** | Option<**String**> | Valid choices for this test (comma-separated) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


