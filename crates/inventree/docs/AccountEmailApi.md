# \AccountEmailApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_email_delete**](AccountEmailApi.md#allauth_account_email_delete) | **DELETE** /api/auth/v1/account/email | Remove an email address
[**allauth_account_email_get**](AccountEmailApi.md#allauth_account_email_get) | **GET** /api/auth/v1/account/email | List email addresses
[**allauth_account_email_patch**](AccountEmailApi.md#allauth_account_email_patch) | **PATCH** /api/auth/v1/account/email | Change primary email address
[**allauth_account_email_post**](AccountEmailApi.md#allauth_account_email_post) | **POST** /api/auth/v1/account/email | Add/Change email address 
[**allauth_account_email_put**](AccountEmailApi.md#allauth_account_email_put) | **PUT** /api/auth/v1/account/email | Request email verification



## allauth_account_email_delete

> models::AllauthAccountEmailGet200Response allauth_account_email_delete(allauth_account_email_put_request)
Remove an email address

Used to remove an email address. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_email_put_request** | Option<[**AllauthAccountEmailPutRequest**](AllauthAccountEmailPutRequest.md)> |  |  |

### Return type

[**models::AllauthAccountEmailGet200Response**](allauth_account_email_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_email_get

> models::AllauthAccountEmailGet200Response allauth_account_email_get()
List email addresses

Retrieves the list of email addresses of the account. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAccountEmailGet200Response**](allauth_account_email_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_email_patch

> models::AllauthAccountEmailGet200Response allauth_account_email_patch(allauth_account_email_patch_request)
Change primary email address

Used to change primary email address to a different one. Note that only verified email addresses can be marked as primary. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_email_patch_request** | Option<[**AllauthAccountEmailPatchRequest**](AllauthAccountEmailPatchRequest.md)> |  |  |

### Return type

[**models::AllauthAccountEmailGet200Response**](allauth_account_email_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_email_post

> models::AllauthAccountEmailGet200Response allauth_account_email_post(allauth_account_email_put_request)
Add/Change email address 

The following functionality is available:    - Adding a new email address for an already signed in user (`ACCOUNT_CHANGE_EMAIL = False`).   - Change to a new email address for an already signed in user   (`ACCOUNT_CHANGE_EMAIL = True`).   - Change to a new email address during the email verification process at signup (`ACCOUNT_EMAIL_VERIFICATION_SUPPORTS_CHANGE = True`).  In all cases, an email verification mail will be sent containing a link or code that needs to be verified. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_email_put_request** | Option<[**AllauthAccountEmailPutRequest**](AllauthAccountEmailPutRequest.md)> |  |  |

### Return type

[**models::AllauthAccountEmailGet200Response**](allauth_account_email_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_email_put

> models::AllauthAuthEmailVerifyResendPost200Response allauth_account_email_put(allauth_account_email_put_request)
Request email verification

Requests for (another) email verification email to be sent. Note that sending emails is rate limited, so when you send too many requests the email will not be sent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_email_put_request** | Option<[**AllauthAccountEmailPutRequest**](AllauthAccountEmailPutRequest.md)> |  |  |

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

