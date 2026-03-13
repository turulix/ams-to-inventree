# \AccountPasswordApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_password_change_post**](AccountPasswordApi.md#allauth_account_password_change_post) | **POST** /api/auth/v1/account/password/change | Change password



## allauth_account_password_change_post

> allauth_account_password_change_post(allauth_account_password_change_post_request)
Change password

In order to change the password of an account, the current and new password must be provider.  However, accounts that were created by signing up using a third-party provider do not have a password set. In that case, the current password is not required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_password_change_post_request** | Option<[**AllauthAccountPasswordChangePostRequest**](AllauthAccountPasswordChangePostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

