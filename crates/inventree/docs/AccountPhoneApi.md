# \AccountPhoneApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allauth_account_phone_get**](AccountPhoneApi.md#allauth_account_phone_get) | **GET** /api/auth/v1/account/phone | Get the phone number
[**allauth_account_phone_post**](AccountPhoneApi.md#allauth_account_phone_post) | **POST** /api/auth/v1/account/phone | Change the phone number 



## allauth_account_phone_get

> models::AllauthPhoneNumbersResponse allauth_account_phone_get()
Get the phone number

Retrieves the phone number of the account, if any. Note that while the endpoint returns a list of phone numbers, at most one entry is returned. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AllauthPhoneNumbersResponse**](allauth.PhoneNumbersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allauth_account_phone_post

> models::AllauthPhoneNumberChangeResponse allauth_account_phone_post(allauth_account_phone_post_request)
Change the phone number 

The following functionality is available:  - Initiate the phone number change process for signed in users. - Change to a new phone number during the phone number verification   process at signup for unauthenticated users. Note that this requires:   `ACCOUNT_PHONE_VERIFICATION_SUPPORTS_CHANGE = True`.  In both cases, after posting a new phone number, proceed with the phone verification endpoint to confirm the change of the phone number by posting the verification code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allauth_account_phone_post_request** | Option<[**AllauthAccountPhonePostRequest**](AllauthAccountPhonePostRequest.md)> |  |  |

### Return type

[**models::AllauthPhoneNumberChangeResponse**](allauth.PhoneNumberChangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

