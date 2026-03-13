# \AuthenticationAccountApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_auth_email_verify_get**](AuthenticationAccountApi.md#allauth_auth_email_verify_get) | **GET** /api/auth/v1/auth/email/verify | Get email verification information
[**allauth_auth_email_verify_post**](AuthenticationAccountApi.md#allauth_auth_email_verify_post) | **POST** /api/auth/v1/auth/email/verify | Verify an email
[**allauth_auth_email_verify_resend_post**](AuthenticationAccountApi.md#allauth_auth_email_verify_resend_post) | **POST** /api/auth/v1/auth/email/verify/resend | Resend email verification code
[**allauth_auth_login_post**](AuthenticationAccountApi.md#allauth_auth_login_post) | **POST** /api/auth/v1/auth/login | Login
[**allauth_auth_phone_verify_post**](AuthenticationAccountApi.md#allauth_auth_phone_verify_post) | **POST** /api/auth/v1/auth/phone/verify | Verify a phone number
[**allauth_auth_phone_verify_resend_post**](AuthenticationAccountApi.md#allauth_auth_phone_verify_resend_post) | **POST** /api/auth/v1/auth/phone/verify/resend | Resend phone number verification code
[**allauth_auth_reauthenticate_post**](AuthenticationAccountApi.md#allauth_auth_reauthenticate_post) | **POST** /api/auth/v1/auth/reauthenticate | Reauthenticate
[**allauth_auth_signup_post**](AuthenticationAccountApi.md#allauth_auth_signup_post) | **POST** /api/auth/v1/auth/signup | Signup



## allauth_auth_email_verify_get

> models::AllauthEmailVerificationInfo allauth_auth_email_verify_get(x_email_verification_key)
Get email verification information

Obtain email verification information, given the token that was sent to the user by email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_email_verification_key** | **String** | The email verification key | [required] |

### Return type

[**models::AllauthEmailVerificationInfo**](allauth.EmailVerificationInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_email_verify_post

> models::AllauthAuthenticatedResponse allauth_auth_email_verify_post(allauth_verify_email)
Verify an email

Complete the email verification process. Depending on the configuration, email addresses are either verified by opening a link that is sent to their email address, or, by inputting a code that is sent. On the API, both cases are handled identically. Meaning, the required key is either the one from the link, or, the code itself.  Note that a status code of 401 does not imply failure. It indicates that the email verification was successful, yet, the user is still not signed in. For example, in case `ACCOUNT_LOGIN_ON_EMAIL_CONFIRMATION` is set to `False`, a 401 is returned when verifying as part of login/signup. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_verify_email** | Option<[**AllauthVerifyEmail**](AllauthVerifyEmail.md)> |  |  |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_email_verify_resend_post

> models::AllauthAuthEmailVerifyResendPost200Response allauth_auth_email_verify_resend_post()
Resend email verification code

Requests a new email verification code. Requires `ACCOUNT_EMAIL_VERIFICATION_SUPPORTS_RESEND = True`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_login_post

> models::AllauthAuthenticatedResponse allauth_auth_login_post(allauth_login)
Login

Login using a username-password or email-password combination. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_login** | [**AllauthLogin**](AllauthLogin.md) | Login. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_phone_verify_post

> models::AllauthAuthenticatedResponse allauth_auth_phone_verify_post(allauth_verify_phone)
Verify a phone number

Complete the phone number verification process. Note that a status code of 401 does not imply failure. It merely indicates that the phone number verification was successful, yet, the user is still not signed in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_verify_phone** | Option<[**AllauthVerifyPhone**](AllauthVerifyPhone.md)> |  |  |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_phone_verify_resend_post

> models::AllauthAuthEmailVerifyResendPost200Response allauth_auth_phone_verify_resend_post()
Resend phone number verification code

Requests a new phone number verification code. Requires `ACCOUNT_PHONE_VERIFICATION_SUPPORTS_RESEND = True`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthAuthEmailVerifyResendPost200Response**](allauth_auth_email_verify_resend_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_reauthenticate_post

> models::AllauthAuthenticatedResponse allauth_auth_reauthenticate_post(allauth_reauthenticate)
Reauthenticate

In order to safeguard the account, some actions require the user to be recently authenticated.  If you try to perform such an action without having been recently authenticated, a `401` status is returned, listing flows that can be performed to reauthenticate. One such flow is the flow with ID `reauthenticate`, which allows for the user to input the password. This is the endpoint related towards that flow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_reauthenticate** | [**AllauthReauthenticate**](AllauthReauthenticate.md) | Reauthenticate. | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_auth_signup_post

> models::AllauthAuthenticatedResponse allauth_auth_signup_post(allauth_signup)
Signup

Whether or not `username`, `email`, `phone` or combination of those are required depends on the configuration of django-allauth. Additionally, if a custom signup form is used there may be other custom properties required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_signup** | [**AllauthSignup**](AllauthSignup.md) | Signup | [required] |

### Return type

[**models::AllauthAuthenticatedResponse**](allauth.AuthenticatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

