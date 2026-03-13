# \AuthenticationPasswordResetApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_password_request_post**](AuthenticationPasswordResetApi.md#allauth_auth_password_request_post) | **POST** /api/auth/v1/auth/password/request | Request password
[**allauth_auth_password_reset_get**](AuthenticationPasswordResetApi.md#allauth_auth_password_reset_get) | **GET** /api/auth/v1/auth/password/reset | Get password reset information
[**allauth_auth_password_reset_post**](AuthenticationPasswordResetApi.md#allauth_auth_password_reset_post) | **POST** /api/auth/v1/auth/password/reset | Reset password



## allauth_auth_password_request_post

> models::AllauthAuthEmailVerifyResendPost200Response allauth_auth_password_request_post(allauth_request_password)
Request password

Initiates the password reset procedure. Depending on whether or not `ACCOUNT_PASSWORD_RESET_BY_CODE_ENABLED` is `True`, the procedure is either stateless or stateful.  In case codes are used, it is stateful, and a new `password_reset_by_code` flow is started. In this case, on a successful password reset request, you will receive a 401 indicating the pending status of this flow.  In case password reset is configured to use (stateless) links, you will receive a 200 on a successful password reset request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_request_password** | [**AllauthRequestPassword**](AllauthRequestPassword.md) | Request password. | [required] |

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_password_reset_get

> models::AllauthAuthPasswordResetGet200Response allauth_auth_password_reset_get(x_password_reset_key)
Get password reset information

Used to obtain information on and validate a password reset key.  The key passed is either the key encoded in the password reset URL that the user has received per email, or, the password reset code in case of `ACCOUNT_PASSWORD_RESET_BY_CODE_ENABLED`. Note that in case of a code, the number of requests you can make is limited (by `ACCOUNT_PASSWORD_RESET_BY_CODE_MAX_ATTEMPTS`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_password_reset_key** | **String** | The password reset key | [required] |

### Return type

[**models::AllauthAuthPasswordResetGet200Response**](allauth_auth_password_reset_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_password_reset_post

> models::AllauthAuthenticatedResponse allauth_auth_password_reset_post(allauth_reset_password)
Reset password

Perform the password reset, by handing over the password reset key and the new password. After successfully completing the password reset, the user is either logged in (in case `ACCOUNT_LOGIN_ON_PASSWORD_RESET` is `True`), or, the user will need to proceed to the login page.  In case of the former, a `200` status code is returned, in case of the latter a 401. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_reset_password** | Option<[**AllauthResetPassword**](AllauthResetPassword.md)> |  |  |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

