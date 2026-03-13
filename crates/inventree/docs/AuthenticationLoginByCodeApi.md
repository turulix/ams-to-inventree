# \AuthenticationLoginByCodeApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_code_confirm_post**](AuthenticationLoginByCodeApi.md#allauth_auth_code_confirm_post) | **POST** /api/auth/v1/auth/code/confirm | Confirm login code
[**allauth_auth_code_request_post**](AuthenticationLoginByCodeApi.md#allauth_auth_code_request_post) | **POST** /api/auth/v1/auth/code/request | Request login code



## allauth_auth_code_confirm_post

> models::AllauthAuthenticatedResponse allauth_auth_code_confirm_post(allauth_confirm_login_code)
Confirm login code

Use this endpoint to pass along the received \"special\" login code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_confirm_login_code** | [**AllauthConfirmLoginCode**](AllauthConfirmLoginCode.md) |  | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_code_request_post

> allauth_auth_code_request_post(allauth_request_login_code)
Request login code

Request a \"special\" login code that is sent to the user by email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_request_login_code** | [**AllauthRequestLoginCode**](AllauthRequestLoginCode.md) | Request a login code. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

