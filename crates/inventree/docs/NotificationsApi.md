# \NotificationsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notifications_bulk_destroy**](NotificationsApi.md#notifications_bulk_destroy) | **DELETE** /api/notifications/ | 
[**notifications_destroy**](NotificationsApi.md#notifications_destroy) | **DELETE** /api/notifications/{id}/ | 
[**notifications_list**](NotificationsApi.md#notifications_list) | **GET** /api/notifications/ | 
[**notifications_partial_update**](NotificationsApi.md#notifications_partial_update) | **PATCH** /api/notifications/{id}/ | 
[**notifications_readall_retrieve**](NotificationsApi.md#notifications_readall_retrieve) | **GET** /api/notifications/readall/ | 
[**notifications_retrieve**](NotificationsApi.md#notifications_retrieve) | **GET** /api/notifications/{id}/ | 
[**notifications_update**](NotificationsApi.md#notifications_update) | **PUT** /api/notifications/{id}/ | 



## notifications_bulk_destroy

> notifications_bulk_destroy(bulk_request)


Perform a DELETE operation against this list endpoint.  Note that the typical DRF list endpoint does not support DELETE, so this method is provided as a custom implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_request** | [**BulkRequest**](BulkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_destroy

> notifications_destroy(id)


Detail view for an individual notification object.  - User can only view / delete their own notification objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_list

> models::PaginatedNotificationMessageList notifications_list(limit, category, offset, ordering, read, search)


List view for all notifications of the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**category** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**read** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: message, name. |  |

### Return type

[**models::PaginatedNotificationMessageList**](PaginatedNotificationMessageList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_partial_update

> models::NotificationMessage notifications_partial_update(id, patched_notification_message)


Detail view for an individual notification object.  - User can only view / delete their own notification objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_notification_message** | Option<[**PatchedNotificationMessage**](PatchedNotificationMessage.md)> |  |  |

### Return type

[**models::NotificationMessage**](NotificationMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_readall_retrieve

> models::NotificationMessage notifications_readall_retrieve()


Set all messages for the current user as read.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NotificationMessage**](NotificationMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_retrieve

> models::NotificationMessage notifications_retrieve(id)


Detail view for an individual notification object.  - User can only view / delete their own notification objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NotificationMessage**](NotificationMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notifications_update

> models::NotificationMessage notifications_update(id, notification_message)


Detail view for an individual notification object.  - User can only view / delete their own notification objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**notification_message** | [**NotificationMessage**](NotificationMessage.md) |  | [required] |

### Return type

[**models::NotificationMessage**](NotificationMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

