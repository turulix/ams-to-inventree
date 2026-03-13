# \OrderApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**order_po_cancel_create**](OrderApi.md#order_po_cancel_create) | **POST** /api/order/po/{id}/cancel/ | 
[**order_po_complete_create**](OrderApi.md#order_po_complete_create) | **POST** /api/order/po/{id}/complete/ | 
[**order_po_create**](OrderApi.md#order_po_create) | **POST** /api/order/po/ | 
[**order_po_destroy**](OrderApi.md#order_po_destroy) | **DELETE** /api/order/po/{id}/ | 
[**order_po_extra_line_create**](OrderApi.md#order_po_extra_line_create) | **POST** /api/order/po-extra-line/ | 
[**order_po_extra_line_destroy**](OrderApi.md#order_po_extra_line_destroy) | **DELETE** /api/order/po-extra-line/{id}/ | 
[**order_po_extra_line_list**](OrderApi.md#order_po_extra_line_list) | **GET** /api/order/po-extra-line/ | 
[**order_po_extra_line_partial_update**](OrderApi.md#order_po_extra_line_partial_update) | **PATCH** /api/order/po-extra-line/{id}/ | 
[**order_po_extra_line_retrieve**](OrderApi.md#order_po_extra_line_retrieve) | **GET** /api/order/po-extra-line/{id}/ | 
[**order_po_extra_line_update**](OrderApi.md#order_po_extra_line_update) | **PUT** /api/order/po-extra-line/{id}/ | 
[**order_po_hold_create**](OrderApi.md#order_po_hold_create) | **POST** /api/order/po/{id}/hold/ | 
[**order_po_issue_create**](OrderApi.md#order_po_issue_create) | **POST** /api/order/po/{id}/issue/ | 
[**order_po_line_bulk_destroy**](OrderApi.md#order_po_line_bulk_destroy) | **DELETE** /api/order/po-line/ | 
[**order_po_line_create**](OrderApi.md#order_po_line_create) | **POST** /api/order/po-line/ | 
[**order_po_line_destroy**](OrderApi.md#order_po_line_destroy) | **DELETE** /api/order/po-line/{id}/ | 
[**order_po_line_list**](OrderApi.md#order_po_line_list) | **GET** /api/order/po-line/ | 
[**order_po_line_partial_update**](OrderApi.md#order_po_line_partial_update) | **PATCH** /api/order/po-line/{id}/ | 
[**order_po_line_retrieve**](OrderApi.md#order_po_line_retrieve) | **GET** /api/order/po-line/{id}/ | 
[**order_po_line_update**](OrderApi.md#order_po_line_update) | **PUT** /api/order/po-line/{id}/ | 
[**order_po_list**](OrderApi.md#order_po_list) | **GET** /api/order/po/ | 
[**order_po_partial_update**](OrderApi.md#order_po_partial_update) | **PATCH** /api/order/po/{id}/ | 
[**order_po_receive_create**](OrderApi.md#order_po_receive_create) | **POST** /api/order/po/{id}/receive/ | 
[**order_po_retrieve**](OrderApi.md#order_po_retrieve) | **GET** /api/order/po/{id}/ | 
[**order_po_status_retrieve**](OrderApi.md#order_po_status_retrieve) | **GET** /api/order/po/status/ | 
[**order_po_update**](OrderApi.md#order_po_update) | **PUT** /api/order/po/{id}/ | 
[**order_ro_cancel_create**](OrderApi.md#order_ro_cancel_create) | **POST** /api/order/ro/{id}/cancel/ | 
[**order_ro_complete_create**](OrderApi.md#order_ro_complete_create) | **POST** /api/order/ro/{id}/complete/ | 
[**order_ro_create**](OrderApi.md#order_ro_create) | **POST** /api/order/ro/ | 
[**order_ro_destroy**](OrderApi.md#order_ro_destroy) | **DELETE** /api/order/ro/{id}/ | 
[**order_ro_extra_line_create**](OrderApi.md#order_ro_extra_line_create) | **POST** /api/order/ro-extra-line/ | 
[**order_ro_extra_line_destroy**](OrderApi.md#order_ro_extra_line_destroy) | **DELETE** /api/order/ro-extra-line/{id}/ | 
[**order_ro_extra_line_list**](OrderApi.md#order_ro_extra_line_list) | **GET** /api/order/ro-extra-line/ | 
[**order_ro_extra_line_partial_update**](OrderApi.md#order_ro_extra_line_partial_update) | **PATCH** /api/order/ro-extra-line/{id}/ | 
[**order_ro_extra_line_retrieve**](OrderApi.md#order_ro_extra_line_retrieve) | **GET** /api/order/ro-extra-line/{id}/ | 
[**order_ro_extra_line_update**](OrderApi.md#order_ro_extra_line_update) | **PUT** /api/order/ro-extra-line/{id}/ | 
[**order_ro_hold_create**](OrderApi.md#order_ro_hold_create) | **POST** /api/order/ro/{id}/hold/ | 
[**order_ro_issue_create**](OrderApi.md#order_ro_issue_create) | **POST** /api/order/ro/{id}/issue/ | 
[**order_ro_line_create**](OrderApi.md#order_ro_line_create) | **POST** /api/order/ro-line/ | 
[**order_ro_line_destroy**](OrderApi.md#order_ro_line_destroy) | **DELETE** /api/order/ro-line/{id}/ | 
[**order_ro_line_list**](OrderApi.md#order_ro_line_list) | **GET** /api/order/ro-line/ | 
[**order_ro_line_partial_update**](OrderApi.md#order_ro_line_partial_update) | **PATCH** /api/order/ro-line/{id}/ | 
[**order_ro_line_retrieve**](OrderApi.md#order_ro_line_retrieve) | **GET** /api/order/ro-line/{id}/ | 
[**order_ro_line_status_retrieve**](OrderApi.md#order_ro_line_status_retrieve) | **GET** /api/order/ro-line/status/ | 
[**order_ro_line_update**](OrderApi.md#order_ro_line_update) | **PUT** /api/order/ro-line/{id}/ | 
[**order_ro_list**](OrderApi.md#order_ro_list) | **GET** /api/order/ro/ | 
[**order_ro_partial_update**](OrderApi.md#order_ro_partial_update) | **PATCH** /api/order/ro/{id}/ | 
[**order_ro_receive_create**](OrderApi.md#order_ro_receive_create) | **POST** /api/order/ro/{id}/receive/ | 
[**order_ro_retrieve**](OrderApi.md#order_ro_retrieve) | **GET** /api/order/ro/{id}/ | 
[**order_ro_status_retrieve**](OrderApi.md#order_ro_status_retrieve) | **GET** /api/order/ro/status/ | 
[**order_ro_update**](OrderApi.md#order_ro_update) | **PUT** /api/order/ro/{id}/ | 
[**order_so_allocate_create**](OrderApi.md#order_so_allocate_create) | **POST** /api/order/so/{id}/allocate/ | 
[**order_so_allocate_serials_create**](OrderApi.md#order_so_allocate_serials_create) | **POST** /api/order/so/{id}/allocate-serials/ | 
[**order_so_allocation_bulk_partial_update**](OrderApi.md#order_so_allocation_bulk_partial_update) | **PATCH** /api/order/so-allocation/ | 
[**order_so_allocation_bulk_update**](OrderApi.md#order_so_allocation_bulk_update) | **PUT** /api/order/so-allocation/ | 
[**order_so_allocation_destroy**](OrderApi.md#order_so_allocation_destroy) | **DELETE** /api/order/so-allocation/{id}/ | 
[**order_so_allocation_list**](OrderApi.md#order_so_allocation_list) | **GET** /api/order/so-allocation/ | 
[**order_so_allocation_partial_update**](OrderApi.md#order_so_allocation_partial_update) | **PATCH** /api/order/so-allocation/{id}/ | 
[**order_so_allocation_retrieve**](OrderApi.md#order_so_allocation_retrieve) | **GET** /api/order/so-allocation/{id}/ | 
[**order_so_allocation_update**](OrderApi.md#order_so_allocation_update) | **PUT** /api/order/so-allocation/{id}/ | 
[**order_so_cancel_create**](OrderApi.md#order_so_cancel_create) | **POST** /api/order/so/{id}/cancel/ | 
[**order_so_complete_create**](OrderApi.md#order_so_complete_create) | **POST** /api/order/so/{id}/complete/ | 
[**order_so_create**](OrderApi.md#order_so_create) | **POST** /api/order/so/ | 
[**order_so_destroy**](OrderApi.md#order_so_destroy) | **DELETE** /api/order/so/{id}/ | 
[**order_so_extra_line_create**](OrderApi.md#order_so_extra_line_create) | **POST** /api/order/so-extra-line/ | 
[**order_so_extra_line_destroy**](OrderApi.md#order_so_extra_line_destroy) | **DELETE** /api/order/so-extra-line/{id}/ | 
[**order_so_extra_line_list**](OrderApi.md#order_so_extra_line_list) | **GET** /api/order/so-extra-line/ | 
[**order_so_extra_line_partial_update**](OrderApi.md#order_so_extra_line_partial_update) | **PATCH** /api/order/so-extra-line/{id}/ | 
[**order_so_extra_line_retrieve**](OrderApi.md#order_so_extra_line_retrieve) | **GET** /api/order/so-extra-line/{id}/ | 
[**order_so_extra_line_update**](OrderApi.md#order_so_extra_line_update) | **PUT** /api/order/so-extra-line/{id}/ | 
[**order_so_hold_create**](OrderApi.md#order_so_hold_create) | **POST** /api/order/so/{id}/hold/ | 
[**order_so_issue_create**](OrderApi.md#order_so_issue_create) | **POST** /api/order/so/{id}/issue/ | 
[**order_so_line_create**](OrderApi.md#order_so_line_create) | **POST** /api/order/so-line/ | 
[**order_so_line_destroy**](OrderApi.md#order_so_line_destroy) | **DELETE** /api/order/so-line/{id}/ | 
[**order_so_line_list**](OrderApi.md#order_so_line_list) | **GET** /api/order/so-line/ | 
[**order_so_line_partial_update**](OrderApi.md#order_so_line_partial_update) | **PATCH** /api/order/so-line/{id}/ | 
[**order_so_line_retrieve**](OrderApi.md#order_so_line_retrieve) | **GET** /api/order/so-line/{id}/ | 
[**order_so_line_update**](OrderApi.md#order_so_line_update) | **PUT** /api/order/so-line/{id}/ | 
[**order_so_list**](OrderApi.md#order_so_list) | **GET** /api/order/so/ | 
[**order_so_partial_update**](OrderApi.md#order_so_partial_update) | **PATCH** /api/order/so/{id}/ | 
[**order_so_retrieve**](OrderApi.md#order_so_retrieve) | **GET** /api/order/so/{id}/ | 
[**order_so_shipment_create**](OrderApi.md#order_so_shipment_create) | **POST** /api/order/so/shipment/ | 
[**order_so_shipment_destroy**](OrderApi.md#order_so_shipment_destroy) | **DELETE** /api/order/so/shipment/{id}/ | 
[**order_so_shipment_list**](OrderApi.md#order_so_shipment_list) | **GET** /api/order/so/shipment/ | 
[**order_so_shipment_partial_update**](OrderApi.md#order_so_shipment_partial_update) | **PATCH** /api/order/so/shipment/{id}/ | 
[**order_so_shipment_retrieve**](OrderApi.md#order_so_shipment_retrieve) | **GET** /api/order/so/shipment/{id}/ | 
[**order_so_shipment_ship_create**](OrderApi.md#order_so_shipment_ship_create) | **POST** /api/order/so/shipment/{id}/ship/ | 
[**order_so_shipment_update**](OrderApi.md#order_so_shipment_update) | **PUT** /api/order/so/shipment/{id}/ | 
[**order_so_status_retrieve**](OrderApi.md#order_so_status_retrieve) | **GET** /api/order/so/status/ | 
[**order_so_update**](OrderApi.md#order_so_update) | **PUT** /api/order/so/{id}/ | 



## order_po_cancel_create

> order_po_cancel_create(id)


API endpoint to 'cancel' a purchase order.  The purchase order must be in a state which can be cancelled

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


## order_po_complete_create

> models::PurchaseOrderComplete order_po_complete_create(id, purchase_order_complete)


API endpoint to 'complete' a purchase order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**purchase_order_complete** | Option<[**PurchaseOrderComplete**](PurchaseOrderComplete.md)> |  |  |

### Return type

[**models::PurchaseOrderComplete**](PurchaseOrderComplete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_create

> models::PurchaseOrder order_po_create(purchase_order)


API endpoint for accessing a list of PurchaseOrder objects.  - GET: Return list of PurchaseOrder objects (with filters) - POST: Create a new PurchaseOrder object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order** | [**PurchaseOrder**](PurchaseOrder.md) |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_destroy

> order_po_destroy(id)


API endpoint for detail view of a PurchaseOrder object.

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


## order_po_extra_line_create

> models::PurchaseOrderExtraLine order_po_extra_line_create(purchase_order_extra_line)


API endpoint for accessing a list of PurchaseOrderExtraLine objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_extra_line** | [**PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md) |  | [required] |

### Return type

[**models::PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_extra_line_destroy

> order_po_extra_line_destroy(id)


API endpoint for detail view of a PurchaseOrderExtraLine object.

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


## order_po_extra_line_list

> models::PaginatedPurchaseOrderExtraLineList order_po_extra_line_list(limit, offset, order, order_detail, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> |  |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, notes, quantity, reference. |  |

### Return type

[**models::PaginatedPurchaseOrderExtraLineList**](PaginatedPurchaseOrderExtraLineList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_extra_line_partial_update

> models::PurchaseOrderExtraLine order_po_extra_line_partial_update(id, patched_purchase_order_extra_line)


API endpoint for detail view of a PurchaseOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_purchase_order_extra_line** | Option<[**PatchedPurchaseOrderExtraLine**](PatchedPurchaseOrderExtraLine.md)> |  |  |

### Return type

[**models::PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_extra_line_retrieve

> models::PurchaseOrderExtraLine order_po_extra_line_retrieve(id)


API endpoint for detail view of a PurchaseOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_extra_line_update

> models::PurchaseOrderExtraLine order_po_extra_line_update(id, purchase_order_extra_line)


API endpoint for detail view of a PurchaseOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**purchase_order_extra_line** | [**PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md) |  | [required] |

### Return type

[**models::PurchaseOrderExtraLine**](PurchaseOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_hold_create

> order_po_hold_create(id)


API endpoint to place a PurchaseOrder on hold.

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


## order_po_issue_create

> order_po_issue_create(id)


API endpoint to 'issue' (place) a PurchaseOrder.

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


## order_po_line_bulk_destroy

> order_po_line_bulk_destroy(bulk_request)


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


## order_po_line_create

> models::PurchaseOrderLineItem order_po_line_create(purchase_order_line_item)


API endpoint for accessing a list of PurchaseOrderLineItem objects.  - GET: Return a list of PurchaseOrder Line Item objects - POST: Create a new PurchaseOrderLineItem object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_line_item** | [**PurchaseOrderLineItem**](PurchaseOrderLineItem.md) |  | [required] |

### Return type

[**models::PurchaseOrderLineItem**](PurchaseOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_line_destroy

> order_po_line_destroy(id)


Detail API endpoint for PurchaseOrderLineItem object.

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


## order_po_line_list

> models::PaginatedPurchaseOrderLineItemList order_po_line_list(limit, base_part, has_pricing, include_variants, offset, order, order_complete, order_detail, order_status, ordering, part, part_detail, pending, received, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**base_part** | Option<**i32**> | Internal Part |  |
**has_pricing** | Option<**bool**> | Has Pricing |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> | Order |  |
**order_complete** | Option<**bool**> | Order Complete |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**order_status** | Option<**i32**> | Order Status |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> | Supplier Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**pending** | Option<**bool**> | Order Pending |  |
**received** | Option<**bool**> | Items Received |  |
**search** | Option<**String**> | A search term. Searched fields: part__SKU, part__manufacturer_part__MPN, part__part__description, part__part__name, reference. |  |

### Return type

[**models::PaginatedPurchaseOrderLineItemList**](PaginatedPurchaseOrderLineItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_line_partial_update

> models::PurchaseOrderLineItem order_po_line_partial_update(id, patched_purchase_order_line_item)


Detail API endpoint for PurchaseOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_purchase_order_line_item** | Option<[**PatchedPurchaseOrderLineItem**](PatchedPurchaseOrderLineItem.md)> |  |  |

### Return type

[**models::PurchaseOrderLineItem**](PurchaseOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_line_retrieve

> models::PurchaseOrderLineItem order_po_line_retrieve(id, order_detail, part_detail)


Detail API endpoint for PurchaseOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]

### Return type

[**models::PurchaseOrderLineItem**](PurchaseOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_line_update

> models::PurchaseOrderLineItem order_po_line_update(id, purchase_order_line_item)


Detail API endpoint for PurchaseOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**purchase_order_line_item** | [**PurchaseOrderLineItem**](PurchaseOrderLineItem.md) |  | [required] |

### Return type

[**models::PurchaseOrderLineItem**](PurchaseOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_list

> models::PaginatedPurchaseOrderList order_po_list(limit, assigned_to, assigned_to_me, completed_after, completed_before, created_after, created_before, created_by, external_build, has_project_code, has_start_date, has_target_date, max_date, min_date, offset, ordering, outstanding, overdue, part, project_code, reference, search, start_date_after, start_date_before, status, supplier, supplier_detail, supplier_part, target_date_after, target_date_before)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**assigned_to** | Option<**i32**> | Responsible |  |
**assigned_to_me** | Option<**bool**> | Assigned to me |  |
**completed_after** | Option<**String**> | Completed After |  |
**completed_before** | Option<**String**> | Completed Before |  |
**created_after** | Option<**String**> | Created After |  |
**created_before** | Option<**String**> | Created Before |  |
**created_by** | Option<**i32**> | Created By |  |
**external_build** | Option<**i32**> | External Build Order |  |
**has_project_code** | Option<**bool**> | Has Project Code |  |
**has_start_date** | Option<**bool**> | Has Start Date |  |
**has_target_date** | Option<**bool**> | Has Target Date |  |
**max_date** | Option<**String**> | Max Date |  |
**min_date** | Option<**String**> | Min Date |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outstanding** | Option<**bool**> | Outstanding |  |
**overdue** | Option<**bool**> | overdue |  |
**part** | Option<**i32**> | Part |  |
**project_code** | Option<**i32**> | Project Code |  |
**reference** | Option<**String**> | Order Reference |  |
**search** | Option<**String**> | A search term. Searched fields: description, project_code__code, reference, supplier__name, supplier_reference. |  |
**start_date_after** | Option<**String**> | Start Date After |  |
**start_date_before** | Option<**String**> | Start Date Before |  |
**status** | Option<**i32**> | Order Status |  |
**supplier** | Option<**i32**> |  |  |
**supplier_detail** | Option<**bool**> | Include detailed information about the supplier in the response |  |[default to false]
**supplier_part** | Option<**i32**> | Supplier Part |  |
**target_date_after** | Option<**String**> | Target Date After |  |
**target_date_before** | Option<**String**> | Target Date Before |  |

### Return type

[**models::PaginatedPurchaseOrderList**](PaginatedPurchaseOrderList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_partial_update

> models::PurchaseOrder order_po_partial_update(id, patched_purchase_order)


API endpoint for detail view of a PurchaseOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_purchase_order** | Option<[**PatchedPurchaseOrder**](PatchedPurchaseOrder.md)> |  |  |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_receive_create

> Vec<models::StockItem> order_po_receive_create(id, purchase_order_receive)


API endpoint to receive stock items against a PurchaseOrder.  - The purchase order is specified in the URL. - Items to receive are specified as a list called \"items\" with the following options:     - line_item: pk of the PO Line item     - supplier_part: pk value of the supplier part     - quantity: quantity to receive     - status: stock item status     - expiry_date: stock item expiry date (optional)     - location: destination for stock item (optional)     - batch_code: the batch code for this stock item     - serial_numbers: serial numbers for this stock item - A global location must also be specified. This is used when no locations are specified for items, and no location is given in the PO line item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**purchase_order_receive** | [**PurchaseOrderReceive**](PurchaseOrderReceive.md) |  | [required] |

### Return type

[**Vec<models::StockItem>**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_retrieve

> models::PurchaseOrder order_po_retrieve(id, supplier_detail)


API endpoint for detail view of a PurchaseOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**supplier_detail** | Option<**bool**> | Include detailed information about the supplier in the response |  |[default to false]

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_status_retrieve

> models::GenericStateClass order_po_status_retrieve()


Retrieve information about a specific status code

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_po_update

> models::PurchaseOrder order_po_update(id, purchase_order)


API endpoint for detail view of a PurchaseOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**purchase_order** | [**PurchaseOrder**](PurchaseOrder.md) |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_cancel_create

> order_ro_cancel_create(id)


API endpoint to cancel a ReturnOrder.

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


## order_ro_complete_create

> order_ro_complete_create(id)


API endpoint to complete a ReturnOrder.

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


## order_ro_create

> models::ReturnOrder order_ro_create(return_order)


API endpoint for accessing a list of ReturnOrder objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order** | [**ReturnOrder**](ReturnOrder.md) |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_destroy

> order_ro_destroy(id)


API endpoint for detail view of a single ReturnOrder object.

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


## order_ro_extra_line_create

> models::ReturnOrderExtraLine order_ro_extra_line_create(return_order_extra_line)


API endpoint for accessing a list of ReturnOrderExtraLine objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_extra_line** | [**ReturnOrderExtraLine**](ReturnOrderExtraLine.md) |  | [required] |

### Return type

[**models::ReturnOrderExtraLine**](ReturnOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_extra_line_destroy

> order_ro_extra_line_destroy(id)


API endpoint for detail view of a ReturnOrderExtraLine object.

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


## order_ro_extra_line_list

> models::PaginatedReturnOrderExtraLineList order_ro_extra_line_list(limit, offset, order, order_detail, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> |  |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, notes, quantity, reference. |  |

### Return type

[**models::PaginatedReturnOrderExtraLineList**](PaginatedReturnOrderExtraLineList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_extra_line_partial_update

> models::ReturnOrderExtraLine order_ro_extra_line_partial_update(id, patched_return_order_extra_line)


API endpoint for detail view of a ReturnOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_return_order_extra_line** | Option<[**PatchedReturnOrderExtraLine**](PatchedReturnOrderExtraLine.md)> |  |  |

### Return type

[**models::ReturnOrderExtraLine**](ReturnOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_extra_line_retrieve

> models::ReturnOrderExtraLine order_ro_extra_line_retrieve(id)


API endpoint for detail view of a ReturnOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReturnOrderExtraLine**](ReturnOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_extra_line_update

> models::ReturnOrderExtraLine order_ro_extra_line_update(id, return_order_extra_line)


API endpoint for detail view of a ReturnOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**return_order_extra_line** | [**ReturnOrderExtraLine**](ReturnOrderExtraLine.md) |  | [required] |

### Return type

[**models::ReturnOrderExtraLine**](ReturnOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_hold_create

> order_ro_hold_create(id)


API endpoint to hold a ReturnOrder.

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


## order_ro_issue_create

> order_ro_issue_create(id)


API endpoint to issue (place) a ReturnOrder.

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


## order_ro_line_create

> models::ReturnOrderLineItem order_ro_line_create(return_order_line_item)


API endpoint for accessing a list of ReturnOrderLineItemList objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_line_item** | [**ReturnOrderLineItem**](ReturnOrderLineItem.md) |  | [required] |

### Return type

[**models::ReturnOrderLineItem**](ReturnOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_line_destroy

> order_ro_line_destroy(id)


API endpoint for detail view of a ReturnOrderLineItem object.

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


## order_ro_line_list

> models::PaginatedReturnOrderLineItemList order_ro_line_list(limit, has_pricing, item, item_detail, offset, order, order_detail, order_status, ordering, outcome, part_detail, received, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**has_pricing** | Option<**bool**> | Has Pricing |  |
**item** | Option<**i32**> |  |  |
**item_detail** | Option<**bool**> | Include detailed information about the item in the response |  |[default to true]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> |  |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**order_status** | Option<**i32**> | Order Status |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outcome** | Option<**i32**> | outcome |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**received** | Option<**bool**> | received |  |
**search** | Option<**String**> | A search term. Searched fields: item__part__description, item__part__name, item__serial, reference. |  |

### Return type

[**models::PaginatedReturnOrderLineItemList**](PaginatedReturnOrderLineItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_line_partial_update

> models::ReturnOrderLineItem order_ro_line_partial_update(id, patched_return_order_line_item)


API endpoint for detail view of a ReturnOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_return_order_line_item** | Option<[**PatchedReturnOrderLineItem**](PatchedReturnOrderLineItem.md)> |  |  |

### Return type

[**models::ReturnOrderLineItem**](ReturnOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_line_retrieve

> models::ReturnOrderLineItem order_ro_line_retrieve(id, item_detail, order_detail, part_detail)


API endpoint for detail view of a ReturnOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**item_detail** | Option<**bool**> | Include detailed information about the item in the response |  |[default to true]
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]

### Return type

[**models::ReturnOrderLineItem**](ReturnOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_line_status_retrieve

> models::GenericStateClass order_ro_line_status_retrieve()


Retrieve information about a specific status code

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_line_update

> models::ReturnOrderLineItem order_ro_line_update(id, return_order_line_item)


API endpoint for detail view of a ReturnOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**return_order_line_item** | [**ReturnOrderLineItem**](ReturnOrderLineItem.md) |  | [required] |

### Return type

[**models::ReturnOrderLineItem**](ReturnOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_list

> models::PaginatedReturnOrderList order_ro_list(limit, assigned_to, assigned_to_me, completed_after, completed_before, created_after, created_before, created_by, customer, customer_detail, has_project_code, has_start_date, has_target_date, include_variants, max_date, min_date, offset, ordering, outstanding, overdue, part, project_code, reference, search, start_date_after, start_date_before, status, target_date_after, target_date_before)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**assigned_to** | Option<**i32**> | Responsible |  |
**assigned_to_me** | Option<**bool**> | Assigned to me |  |
**completed_after** | Option<**String**> | Completed After |  |
**completed_before** | Option<**String**> | Completed Before |  |
**created_after** | Option<**String**> | Created After |  |
**created_before** | Option<**String**> | Created Before |  |
**created_by** | Option<**i32**> | Created By |  |
**customer** | Option<**i32**> |  |  |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]
**has_project_code** | Option<**bool**> | Has Project Code |  |
**has_start_date** | Option<**bool**> | Has Start Date |  |
**has_target_date** | Option<**bool**> | Has Target Date |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**max_date** | Option<**String**> | Max Date |  |
**min_date** | Option<**String**> | Min Date |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outstanding** | Option<**bool**> | Outstanding |  |
**overdue** | Option<**bool**> | overdue |  |
**part** | Option<**i32**> |  |  |
**project_code** | Option<**i32**> | Project Code |  |
**reference** | Option<**String**> | Order Reference |  |
**search** | Option<**String**> | A search term. Searched fields: customer__name, customer_reference, description, project_code__code, reference. |  |
**start_date_after** | Option<**String**> | Start Date After |  |
**start_date_before** | Option<**String**> | Start Date Before |  |
**status** | Option<**i32**> | Order Status |  |
**target_date_after** | Option<**String**> | Target Date After |  |
**target_date_before** | Option<**String**> | Target Date Before |  |

### Return type

[**models::PaginatedReturnOrderList**](PaginatedReturnOrderList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_partial_update

> models::ReturnOrder order_ro_partial_update(id, patched_return_order)


API endpoint for detail view of a single ReturnOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_return_order** | Option<[**PatchedReturnOrder**](PatchedReturnOrder.md)> |  |  |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_receive_create

> models::ReturnOrderReceive order_ro_receive_create(id, return_order_receive)


API endpoint to receive items against a ReturnOrder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**return_order_receive** | [**ReturnOrderReceive**](ReturnOrderReceive.md) |  | [required] |

### Return type

[**models::ReturnOrderReceive**](ReturnOrderReceive.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_retrieve

> models::ReturnOrder order_ro_retrieve(id, customer_detail)


API endpoint for detail view of a single ReturnOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_status_retrieve

> models::GenericStateClass order_ro_status_retrieve()


Retrieve information about a specific status code

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_ro_update

> models::ReturnOrder order_ro_update(id, return_order)


API endpoint for detail view of a single ReturnOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**return_order** | [**ReturnOrder**](ReturnOrder.md) |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocate_create

> models::SalesOrderShipmentAllocation order_so_allocate_create(id, sales_order_shipment_allocation)


API endpoint to allocate stock items against a SalesOrder.  - The SalesOrder is specified in the URL - See the SalesOrderShipmentAllocationSerializer class

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_shipment_allocation** | [**SalesOrderShipmentAllocation**](SalesOrderShipmentAllocation.md) |  | [required] |

### Return type

[**models::SalesOrderShipmentAllocation**](SalesOrderShipmentAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocate_serials_create

> models::SalesOrderSerialAllocation order_so_allocate_serials_create(id, sales_order_serial_allocation)


API endpoint to allocation stock items against a SalesOrder, by specifying serial numbers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_serial_allocation** | [**SalesOrderSerialAllocation**](SalesOrderSerialAllocation.md) |  | [required] |

### Return type

[**models::SalesOrderSerialAllocation**](SalesOrderSerialAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_bulk_partial_update

> models::SalesOrderAllocation order_so_allocation_bulk_partial_update(patched_sales_order_allocation)


Perform a PATCH operation against this list endpoint.  Note that the typical DRF list endpoint does not support PATCH, so this method is provided as a custom implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_sales_order_allocation** | Option<[**PatchedSalesOrderAllocation**](PatchedSalesOrderAllocation.md)> |  |  |

### Return type

[**models::SalesOrderAllocation**](SalesOrderAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_bulk_update

> models::SalesOrderAllocation order_so_allocation_bulk_update(sales_order_allocation)


Perform a PUT operation against this list endpoint.  Simply redirects to the PATCH method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sales_order_allocation** | [**SalesOrderAllocation**](SalesOrderAllocation.md) |  | [required] |

### Return type

[**models::SalesOrderAllocation**](SalesOrderAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_destroy

> order_so_allocation_destroy(id)


API endpoint for detail view of a SalesOrderAllocation object.

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


## order_so_allocation_list

> models::PaginatedSalesOrderAllocationList order_so_allocation_list(limit, assigned_to_shipment, customer_detail, include_variants, item, item_detail, line, location, location_detail, offset, order, order_detail, ordering, outstanding, part, part_detail, search, shipment)


API endpoint for listing SalesOrderAllocation objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**assigned_to_shipment** | Option<**bool**> | Has Shipment |  |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]
**include_variants** | Option<**bool**> | Include Variants |  |
**item** | Option<**i32**> |  |  |
**item_detail** | Option<**bool**> | Include detailed information about the item in the response |  |[default to false]
**line** | Option<**i32**> |  |  |
**location** | Option<**i32**> | Location |  |
**location_detail** | Option<**bool**> | Include detailed information about the stock location in the response |  |[default to false]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> | Order |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outstanding** | Option<**bool**> | Outstanding |  |
**part** | Option<**i32**> | Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: item__batch, item__part__IPN, item__part__name, item__serial. |  |
**shipment** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedSalesOrderAllocationList**](PaginatedSalesOrderAllocationList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_partial_update

> models::SalesOrderAllocation order_so_allocation_partial_update(id, patched_sales_order_allocation)


API endpoint for detail view of a SalesOrderAllocation object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_sales_order_allocation** | Option<[**PatchedSalesOrderAllocation**](PatchedSalesOrderAllocation.md)> |  |  |

### Return type

[**models::SalesOrderAllocation**](SalesOrderAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_retrieve

> models::SalesOrderAllocation order_so_allocation_retrieve(id)


API endpoint for detail view of a SalesOrderAllocation object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::SalesOrderAllocation**](SalesOrderAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_allocation_update

> models::SalesOrderAllocation order_so_allocation_update(id, sales_order_allocation)


API endpoint for detail view of a SalesOrderAllocation object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_allocation** | [**SalesOrderAllocation**](SalesOrderAllocation.md) |  | [required] |

### Return type

[**models::SalesOrderAllocation**](SalesOrderAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_cancel_create

> order_so_cancel_create(id)


API endpoint to cancel a SalesOrder.

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


## order_so_complete_create

> models::SalesOrderComplete order_so_complete_create(id, sales_order_complete)


API endpoint for manually marking a SalesOrder as \"complete\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_complete** | Option<[**SalesOrderComplete**](SalesOrderComplete.md)> |  |  |

### Return type

[**models::SalesOrderComplete**](SalesOrderComplete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_create

> models::SalesOrder order_so_create(sales_order)


API endpoint for accessing a list of SalesOrder objects.  - GET: Return list of SalesOrder objects (with filters) - POST: Create a new SalesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sales_order** | [**SalesOrder**](SalesOrder.md) |  | [required] |

### Return type

[**models::SalesOrder**](SalesOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_destroy

> order_so_destroy(id)


API endpoint for detail view of a SalesOrder object.

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


## order_so_extra_line_create

> models::SalesOrderExtraLine order_so_extra_line_create(sales_order_extra_line)


API endpoint for accessing a list of SalesOrderExtraLine objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sales_order_extra_line** | [**SalesOrderExtraLine**](SalesOrderExtraLine.md) |  | [required] |

### Return type

[**models::SalesOrderExtraLine**](SalesOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_extra_line_destroy

> order_so_extra_line_destroy(id)


API endpoint for detail view of a SalesOrderExtraLine object.

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


## order_so_extra_line_list

> models::PaginatedSalesOrderExtraLineList order_so_extra_line_list(limit, offset, order, order_detail, ordering, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> |  |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: description, notes, quantity, reference. |  |

### Return type

[**models::PaginatedSalesOrderExtraLineList**](PaginatedSalesOrderExtraLineList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_extra_line_partial_update

> models::SalesOrderExtraLine order_so_extra_line_partial_update(id, patched_sales_order_extra_line)


API endpoint for detail view of a SalesOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_sales_order_extra_line** | Option<[**PatchedSalesOrderExtraLine**](PatchedSalesOrderExtraLine.md)> |  |  |

### Return type

[**models::SalesOrderExtraLine**](SalesOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_extra_line_retrieve

> models::SalesOrderExtraLine order_so_extra_line_retrieve(id)


API endpoint for detail view of a SalesOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::SalesOrderExtraLine**](SalesOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_extra_line_update

> models::SalesOrderExtraLine order_so_extra_line_update(id, sales_order_extra_line)


API endpoint for detail view of a SalesOrderExtraLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_extra_line** | [**SalesOrderExtraLine**](SalesOrderExtraLine.md) |  | [required] |

### Return type

[**models::SalesOrderExtraLine**](SalesOrderExtraLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_hold_create

> order_so_hold_create(id)


API endpoint to place a SalesOrder on hold.

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


## order_so_issue_create

> order_so_issue_create(id)


API endpoint to issue a SalesOrder.

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


## order_so_line_create

> models::SalesOrderLineItem order_so_line_create(sales_order_line_item)


API endpoint for accessing a list of SalesOrderLineItem objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sales_order_line_item** | [**SalesOrderLineItem**](SalesOrderLineItem.md) |  | [required] |

### Return type

[**models::SalesOrderLineItem**](SalesOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_line_destroy

> order_so_line_destroy(id)


API endpoint for detail view of a SalesOrderLineItem object.

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


## order_so_line_list

> models::PaginatedSalesOrderLineItemList order_so_line_list(limit, allocated, completed, customer_detail, has_pricing, offset, order, order_complete, order_detail, order_outstanding, order_status, ordering, part, part_detail, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**allocated** | Option<**bool**> | Allocated |  |
**completed** | Option<**bool**> | Completed |  |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]
**has_pricing** | Option<**bool**> | Has Pricing |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> | Order |  |
**order_complete** | Option<**bool**> | Order Complete |  |
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**order_outstanding** | Option<**bool**> | Order Outstanding |  |
**order_status** | Option<**i32**> | Order Status |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: part__name, quantity, reference. |  |

### Return type

[**models::PaginatedSalesOrderLineItemList**](PaginatedSalesOrderLineItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_line_partial_update

> models::SalesOrderLineItem order_so_line_partial_update(id, patched_sales_order_line_item)


API endpoint for detail view of a SalesOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_sales_order_line_item** | Option<[**PatchedSalesOrderLineItem**](PatchedSalesOrderLineItem.md)> |  |  |

### Return type

[**models::SalesOrderLineItem**](SalesOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_line_retrieve

> models::SalesOrderLineItem order_so_line_retrieve(id, customer_detail, order_detail, part_detail)


API endpoint for detail view of a SalesOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]
**order_detail** | Option<**bool**> | Include detailed information about the sales order in the response |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]

### Return type

[**models::SalesOrderLineItem**](SalesOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_line_update

> models::SalesOrderLineItem order_so_line_update(id, sales_order_line_item)


API endpoint for detail view of a SalesOrderLineItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_line_item** | [**SalesOrderLineItem**](SalesOrderLineItem.md) |  | [required] |

### Return type

[**models::SalesOrderLineItem**](SalesOrderLineItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_list

> models::PaginatedSalesOrderList order_so_list(limit, assigned_to, assigned_to_me, completed_after, completed_before, created_after, created_before, created_by, customer, customer_detail, has_project_code, has_start_date, has_target_date, include_variants, max_date, min_date, offset, ordering, outstanding, overdue, part, project_code, reference, search, start_date_after, start_date_before, status, target_date_after, target_date_before)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**assigned_to** | Option<**i32**> | Responsible |  |
**assigned_to_me** | Option<**bool**> | Assigned to me |  |
**completed_after** | Option<**String**> | Completed After |  |
**completed_before** | Option<**String**> | Completed Before |  |
**created_after** | Option<**String**> | Created After |  |
**created_before** | Option<**String**> | Created Before |  |
**created_by** | Option<**i32**> | Created By |  |
**customer** | Option<**i32**> |  |  |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]
**has_project_code** | Option<**bool**> | Has Project Code |  |
**has_start_date** | Option<**bool**> | Has Start Date |  |
**has_target_date** | Option<**bool**> | Has Target Date |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**max_date** | Option<**String**> | Max Date |  |
**min_date** | Option<**String**> | Min Date |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outstanding** | Option<**bool**> | Outstanding |  |
**overdue** | Option<**bool**> | overdue |  |
**part** | Option<**i32**> |  |  |
**project_code** | Option<**i32**> | Project Code |  |
**reference** | Option<**String**> | Order Reference |  |
**search** | Option<**String**> | A search term. Searched fields: customer__name, customer_reference, description, project_code__code, reference. |  |
**start_date_after** | Option<**String**> | Start Date After |  |
**start_date_before** | Option<**String**> | Start Date Before |  |
**status** | Option<**i32**> | Order Status |  |
**target_date_after** | Option<**String**> | Target Date After |  |
**target_date_before** | Option<**String**> | Target Date Before |  |

### Return type

[**models::PaginatedSalesOrderList**](PaginatedSalesOrderList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_partial_update

> models::SalesOrder order_so_partial_update(id, patched_sales_order)


API endpoint for detail view of a SalesOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_sales_order** | Option<[**PatchedSalesOrder**](PatchedSalesOrder.md)> |  |  |

### Return type

[**models::SalesOrder**](SalesOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_retrieve

> models::SalesOrder order_so_retrieve(id, customer_detail)


API endpoint for detail view of a SalesOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**customer_detail** | Option<**bool**> | Include detailed information about the customer in the response |  |[default to false]

### Return type

[**models::SalesOrder**](SalesOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_create

> models::SalesOrderShipment order_so_shipment_create(sales_order_shipment)


API list endpoint for SalesOrderShipment model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sales_order_shipment** | [**SalesOrderShipment**](SalesOrderShipment.md) |  | [required] |

### Return type

[**models::SalesOrderShipment**](SalesOrderShipment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_destroy

> order_so_shipment_destroy(id)


API detail endpoint for SalesOrderShipment model.

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


## order_so_shipment_list

> models::PaginatedSalesOrderShipmentList order_so_shipment_list(limit, checked, delivered, offset, order, order_outstanding, order_status, ordering, search, shipped)


API list endpoint for SalesOrderShipment model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**checked** | Option<**bool**> | checked |  |
**delivered** | Option<**bool**> | delivered |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**order** | Option<**i32**> |  |  |
**order_outstanding** | Option<**bool**> | Order Outstanding |  |
**order_status** | Option<**f64**> | Order Status |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: invoice_number, order__reference, reference, tracking_number. |  |
**shipped** | Option<**bool**> | shipped |  |

### Return type

[**models::PaginatedSalesOrderShipmentList**](PaginatedSalesOrderShipmentList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_partial_update

> models::SalesOrderShipment order_so_shipment_partial_update(id, patched_sales_order_shipment)


API detail endpoint for SalesOrderShipment model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_sales_order_shipment** | Option<[**PatchedSalesOrderShipment**](PatchedSalesOrderShipment.md)> |  |  |

### Return type

[**models::SalesOrderShipment**](SalesOrderShipment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_retrieve

> models::SalesOrderShipment order_so_shipment_retrieve(id)


API detail endpoint for SalesOrderShipment model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::SalesOrderShipment**](SalesOrderShipment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_ship_create

> models::SalesOrderShipmentComplete order_so_shipment_ship_create(id, sales_order_shipment_complete)


API endpoint for completing (shipping) a SalesOrderShipment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_shipment_complete** | Option<[**SalesOrderShipmentComplete**](SalesOrderShipmentComplete.md)> |  |  |

### Return type

[**models::SalesOrderShipmentComplete**](SalesOrderShipmentComplete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_shipment_update

> models::SalesOrderShipment order_so_shipment_update(id, sales_order_shipment)


API detail endpoint for SalesOrderShipment model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order_shipment** | [**SalesOrderShipment**](SalesOrderShipment.md) |  | [required] |

### Return type

[**models::SalesOrderShipment**](SalesOrderShipment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_status_retrieve

> models::GenericStateClass order_so_status_retrieve()


Retrieve information about a specific status code

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_so_update

> models::SalesOrder order_so_update(id, sales_order)


API endpoint for detail view of a SalesOrder object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**sales_order** | [**SalesOrder**](SalesOrder.md) |  | [required] |

### Return type

[**models::SalesOrder**](SalesOrder.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

