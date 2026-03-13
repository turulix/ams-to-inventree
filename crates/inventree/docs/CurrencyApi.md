# \CurrencyApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**currency_exchange_retrieve**](CurrencyApi.md#currency_exchange_retrieve) | **GET** /api/currency/exchange/ | 
[**currency_refresh_create**](CurrencyApi.md#currency_refresh_create) | **POST** /api/currency/refresh/ | 



## currency_exchange_retrieve

> models::CurrencyExchange currency_exchange_retrieve()


Return information on available currency conversions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CurrencyExchange**](CurrencyExchange.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_refresh_create

> currency_refresh_create()


Performing a POST request will update currency exchange rates.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

