# AllauthProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The provider ID.  | 
**name** | **String** | The name of the provider.  | 
**client_id** | Option<**String**> | The client ID (in case of OAuth2 or OpenID Connect based providers)  | [optional]
**openid_configuration_url** | Option<**String**> | The OIDC discovery or well-known URL (in case of OAuth2 or OpenID Connect based providers)  | [optional]
**flows** | [**Vec<models::FlowsEnum>**](FlowsEnum.md) | The authentication flows the provider integration supports.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


