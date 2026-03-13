# \ImporterApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**importer_column_mapping_list**](ImporterApi.md#importer_column_mapping_list) | **GET** /api/importer/column-mapping/ | 
[**importer_column_mapping_partial_update**](ImporterApi.md#importer_column_mapping_partial_update) | **PATCH** /api/importer/column-mapping/{id}/ | 
[**importer_column_mapping_retrieve**](ImporterApi.md#importer_column_mapping_retrieve) | **GET** /api/importer/column-mapping/{id}/ | 
[**importer_column_mapping_update**](ImporterApi.md#importer_column_mapping_update) | **PUT** /api/importer/column-mapping/{id}/ | 
[**importer_models_list**](ImporterApi.md#importer_models_list) | **GET** /api/importer/models/ | 
[**importer_row_bulk_destroy**](ImporterApi.md#importer_row_bulk_destroy) | **DELETE** /api/importer/row/ | 
[**importer_row_destroy**](ImporterApi.md#importer_row_destroy) | **DELETE** /api/importer/row/{id}/ | 
[**importer_row_list**](ImporterApi.md#importer_row_list) | **GET** /api/importer/row/ | 
[**importer_row_partial_update**](ImporterApi.md#importer_row_partial_update) | **PATCH** /api/importer/row/{id}/ | 
[**importer_row_retrieve**](ImporterApi.md#importer_row_retrieve) | **GET** /api/importer/row/{id}/ | 
[**importer_row_update**](ImporterApi.md#importer_row_update) | **PUT** /api/importer/row/{id}/ | 
[**importer_session_accept_fields_create**](ImporterApi.md#importer_session_accept_fields_create) | **POST** /api/importer/session/{id}/accept_fields/ | 
[**importer_session_accept_rows_create**](ImporterApi.md#importer_session_accept_rows_create) | **POST** /api/importer/session/{id}/accept_rows/ | 
[**importer_session_bulk_destroy**](ImporterApi.md#importer_session_bulk_destroy) | **DELETE** /api/importer/session/ | 
[**importer_session_create**](ImporterApi.md#importer_session_create) | **POST** /api/importer/session/ | 
[**importer_session_destroy**](ImporterApi.md#importer_session_destroy) | **DELETE** /api/importer/session/{id}/ | 
[**importer_session_list**](ImporterApi.md#importer_session_list) | **GET** /api/importer/session/ | 
[**importer_session_partial_update**](ImporterApi.md#importer_session_partial_update) | **PATCH** /api/importer/session/{id}/ | 
[**importer_session_retrieve**](ImporterApi.md#importer_session_retrieve) | **GET** /api/importer/session/{id}/ | 
[**importer_session_update**](ImporterApi.md#importer_session_update) | **PUT** /api/importer/session/{id}/ | 



## importer_column_mapping_list

> models::PaginatedDataImportColumnMapList importer_column_mapping_list(limit, offset, ordering, search, session)


API endpoint for accessing a list of DataImportColumnMap objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |
**session** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedDataImportColumnMapList**](PaginatedDataImportColumnMapList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_column_mapping_partial_update

> models::DataImportColumnMap importer_column_mapping_partial_update(id, patched_data_import_column_map)


Detail endpoint for a single DataImportColumnMap object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_data_import_column_map** | Option<[**PatchedDataImportColumnMap**](PatchedDataImportColumnMap.md)> |  |  |

### Return type

[**models::DataImportColumnMap**](DataImportColumnMap.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_column_mapping_retrieve

> models::DataImportColumnMap importer_column_mapping_retrieve(id)


Detail endpoint for a single DataImportColumnMap object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DataImportColumnMap**](DataImportColumnMap.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_column_mapping_update

> models::DataImportColumnMap importer_column_mapping_update(id, data_import_column_map)


Detail endpoint for a single DataImportColumnMap object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**data_import_column_map** | Option<[**DataImportColumnMap**](DataImportColumnMap.md)> |  |  |

### Return type

[**models::DataImportColumnMap**](DataImportColumnMap.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_models_list

> Vec<models::DataImporterModel> importer_models_list()


Return a list of models available for import.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DataImporterModel>**](DataImporterModel.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_row_bulk_destroy

> importer_row_bulk_destroy(bulk_request)


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


## importer_row_destroy

> importer_row_destroy(id)


Detail endpoint for a single DataImportRow object.

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


## importer_row_list

> models::PaginatedDataImportRowList importer_row_list(limit, complete, offset, ordering, search, session, valid)


API endpoint for accessing a list of DataImportRow objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**complete** | Option<**bool**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |
**session** | Option<**i32**> |  |  |
**valid** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedDataImportRowList**](PaginatedDataImportRowList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_row_partial_update

> models::DataImportRow importer_row_partial_update(id, patched_data_import_row)


Detail endpoint for a single DataImportRow object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_data_import_row** | Option<[**PatchedDataImportRow**](PatchedDataImportRow.md)> |  |  |

### Return type

[**models::DataImportRow**](DataImportRow.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_row_retrieve

> models::DataImportRow importer_row_retrieve(id)


Detail endpoint for a single DataImportRow object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DataImportRow**](DataImportRow.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_row_update

> models::DataImportRow importer_row_update(id, data_import_row)


Detail endpoint for a single DataImportRow object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**data_import_row** | Option<[**DataImportRow**](DataImportRow.md)> |  |  |

### Return type

[**models::DataImportRow**](DataImportRow.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_accept_fields_create

> models::DataImportSession importer_session_accept_fields_create(id)


Accept the field mapping for a DataImportSession.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DataImportSession**](DataImportSession.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_accept_rows_create

> models::DataImportAcceptRow importer_session_accept_rows_create(id, data_import_accept_row)


API endpoint to accept the rows for a DataImportSession.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**data_import_accept_row** | [**DataImportAcceptRow**](DataImportAcceptRow.md) |  | [required] |

### Return type

[**models::DataImportAcceptRow**](DataImportAcceptRow.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_bulk_destroy

> importer_session_bulk_destroy(bulk_request)


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


## importer_session_create

> models::DataImportSession importer_session_create(data_import_session)


API endpoint for accessing a list of DataImportSession objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_import_session** | [**DataImportSession**](DataImportSession.md) |  | [required] |

### Return type

[**models::DataImportSession**](DataImportSession.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_destroy

> importer_session_destroy(id)


Detail endpoint for a single DataImportSession object.

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


## importer_session_list

> models::PaginatedDataImportSessionList importer_session_list(limit, model_type, offset, ordering, search, status, user)


API endpoint for accessing a list of DataImportSession objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**model_type** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |
**status** | Option<**i32**> | Import status  * `0` - Initializing * `10` - Mapping Columns * `20` - Importing Data * `30` - Processing Data * `40` - Complete |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedDataImportSessionList**](PaginatedDataImportSessionList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_partial_update

> models::DataImportSession importer_session_partial_update(id, patched_data_import_session)


Detail endpoint for a single DataImportSession object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_data_import_session** | Option<[**PatchedDataImportSession**](PatchedDataImportSession.md)> |  |  |

### Return type

[**models::DataImportSession**](DataImportSession.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_retrieve

> models::DataImportSession importer_session_retrieve(id)


Detail endpoint for a single DataImportSession object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DataImportSession**](DataImportSession.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## importer_session_update

> models::DataImportSession importer_session_update(id, data_import_session)


Detail endpoint for a single DataImportSession object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**data_import_session** | [**DataImportSession**](DataImportSession.md) |  | [required] |

### Return type

[**models::DataImportSession**](DataImportSession.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

