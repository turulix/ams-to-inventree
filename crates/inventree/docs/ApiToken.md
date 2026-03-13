# ApiToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | **String** |  | [readonly]
**expiry** | Option<[**String**](String.md)> | Token expiry date | [optional]
**id** | **i32** |  | [readonly]
**last_seen** | Option<[**String**](String.md)> | Last time the token was used | [optional]
**name** | Option<**String**> | Custom token name | [optional]
**token** | **String** | Provide a redacted version of the token.  The *raw* key value should never be displayed anywhere! | [readonly]
**active** | **bool** | Test if this token is active. | [readonly]
**revoked** | Option<**bool**> | Token has been revoked | [optional]
**user** | Option<**i32**> |  | [optional]
**user_detail** | [**models::User**](User.md) |  | [readonly]
**in_use** | **bool** | Return True if the token is currently used to call the endpoint. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


