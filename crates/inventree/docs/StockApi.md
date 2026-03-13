# \StockApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stock_add_create**](StockApi.md#stock_add_create) | **POST** /api/stock/add/ | 
[**stock_assign_create**](StockApi.md#stock_assign_create) | **POST** /api/stock/assign/ | 
[**stock_bulk_destroy**](StockApi.md#stock_bulk_destroy) | **DELETE** /api/stock/ | 
[**stock_change_status_create**](StockApi.md#stock_change_status_create) | **POST** /api/stock/change_status/ | 
[**stock_convert_create**](StockApi.md#stock_convert_create) | **POST** /api/stock/{id}/convert/ | 
[**stock_count_create**](StockApi.md#stock_count_create) | **POST** /api/stock/count/ | 
[**stock_create**](StockApi.md#stock_create) | **POST** /api/stock/ | 
[**stock_destroy**](StockApi.md#stock_destroy) | **DELETE** /api/stock/{id}/ | 
[**stock_install_create**](StockApi.md#stock_install_create) | **POST** /api/stock/{id}/install/ | 
[**stock_list**](StockApi.md#stock_list) | **GET** /api/stock/ | 
[**stock_location_bulk_partial_update**](StockApi.md#stock_location_bulk_partial_update) | **PATCH** /api/stock/location/ | 
[**stock_location_bulk_update**](StockApi.md#stock_location_bulk_update) | **PUT** /api/stock/location/ | 
[**stock_location_create**](StockApi.md#stock_location_create) | **POST** /api/stock/location/ | 
[**stock_location_destroy**](StockApi.md#stock_location_destroy) | **DELETE** /api/stock/location/{id}/ | 
[**stock_location_list**](StockApi.md#stock_location_list) | **GET** /api/stock/location/ | 
[**stock_location_partial_update**](StockApi.md#stock_location_partial_update) | **PATCH** /api/stock/location/{id}/ | 
[**stock_location_retrieve**](StockApi.md#stock_location_retrieve) | **GET** /api/stock/location/{id}/ | 
[**stock_location_tree_list**](StockApi.md#stock_location_tree_list) | **GET** /api/stock/location/tree/ | 
[**stock_location_type_create**](StockApi.md#stock_location_type_create) | **POST** /api/stock/location-type/ | 
[**stock_location_type_destroy**](StockApi.md#stock_location_type_destroy) | **DELETE** /api/stock/location-type/{id}/ | 
[**stock_location_type_list**](StockApi.md#stock_location_type_list) | **GET** /api/stock/location-type/ | 
[**stock_location_type_partial_update**](StockApi.md#stock_location_type_partial_update) | **PATCH** /api/stock/location-type/{id}/ | 
[**stock_location_type_retrieve**](StockApi.md#stock_location_type_retrieve) | **GET** /api/stock/location-type/{id}/ | 
[**stock_location_type_update**](StockApi.md#stock_location_type_update) | **PUT** /api/stock/location-type/{id}/ | 
[**stock_location_update**](StockApi.md#stock_location_update) | **PUT** /api/stock/location/{id}/ | 
[**stock_merge_create**](StockApi.md#stock_merge_create) | **POST** /api/stock/merge/ | 
[**stock_partial_update**](StockApi.md#stock_partial_update) | **PATCH** /api/stock/{id}/ | 
[**stock_remove_create**](StockApi.md#stock_remove_create) | **POST** /api/stock/remove/ | 
[**stock_retrieve**](StockApi.md#stock_retrieve) | **GET** /api/stock/{id}/ | 
[**stock_return_create**](StockApi.md#stock_return_create) | **POST** /api/stock/return/ | 
[**stock_serial_numbers_retrieve**](StockApi.md#stock_serial_numbers_retrieve) | **GET** /api/stock/{id}/serial-numbers/ | 
[**stock_serialize_create**](StockApi.md#stock_serialize_create) | **POST** /api/stock/{id}/serialize/ | 
[**stock_status_retrieve**](StockApi.md#stock_status_retrieve) | **GET** /api/stock/status/ | 
[**stock_test_bulk_destroy**](StockApi.md#stock_test_bulk_destroy) | **DELETE** /api/stock/test/ | 
[**stock_test_create**](StockApi.md#stock_test_create) | **POST** /api/stock/test/ | 
[**stock_test_destroy**](StockApi.md#stock_test_destroy) | **DELETE** /api/stock/test/{id}/ | 
[**stock_test_list**](StockApi.md#stock_test_list) | **GET** /api/stock/test/ | 
[**stock_test_partial_update**](StockApi.md#stock_test_partial_update) | **PATCH** /api/stock/test/{id}/ | 
[**stock_test_retrieve**](StockApi.md#stock_test_retrieve) | **GET** /api/stock/test/{id}/ | 
[**stock_test_update**](StockApi.md#stock_test_update) | **PUT** /api/stock/test/{id}/ | 
[**stock_track_list**](StockApi.md#stock_track_list) | **GET** /api/stock/track/ | 
[**stock_track_retrieve**](StockApi.md#stock_track_retrieve) | **GET** /api/stock/track/{id}/ | 
[**stock_track_status_retrieve**](StockApi.md#stock_track_status_retrieve) | **GET** /api/stock/track/status/ | 
[**stock_transfer_create**](StockApi.md#stock_transfer_create) | **POST** /api/stock/transfer/ | 
[**stock_uninstall_create**](StockApi.md#stock_uninstall_create) | **POST** /api/stock/{id}/uninstall/ | 
[**stock_update**](StockApi.md#stock_update) | **PUT** /api/stock/{id}/ | 



## stock_add_create

> models::StockAdd stock_add_create(stock_add)


Endpoint for adding a quantity of stock to an existing StockItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_add** | [**StockAdd**](StockAdd.md) |  | [required] |

### Return type

[**models::StockAdd**](StockAdd.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_assign_create

> models::StockAssignment stock_assign_create(stock_assignment)


API endpoint for assigning stock to a particular customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_assignment** | [**StockAssignment**](StockAssignment.md) |  | [required] |

### Return type

[**models::StockAssignment**](StockAssignment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_bulk_destroy

> stock_bulk_destroy(bulk_request)


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


## stock_change_status_create

> models::StockChangeStatus stock_change_status_create(stock_change_status)


API endpoint to change the status code of multiple StockItem objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_change_status** | [**StockChangeStatus**](StockChangeStatus.md) |  | [required] |

### Return type

[**models::StockChangeStatus**](StockChangeStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_convert_create

> models::ConvertStockItem stock_convert_create(id, convert_stock_item)


API endpoint for converting a stock item to a variant part.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**convert_stock_item** | [**ConvertStockItem**](ConvertStockItem.md) |  | [required] |

### Return type

[**models::ConvertStockItem**](ConvertStockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_count_create

> models::StockCount stock_count_create(stock_count)


Endpoint for counting stock (performing a stocktake).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_count** | [**StockCount**](StockCount.md) |  | [required] |

### Return type

[**models::StockCount**](StockCount.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_create

> Vec<models::StockItem> stock_create(stock_item)


API endpoint for list view of Stock objects.  - GET: Return a list of all StockItem objects (with optional query filters) - POST: Create a new StockItem - DELETE: Delete multiple StockItem objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_item** | [**StockItem**](StockItem.md) |  | [required] |

### Return type

[**Vec<models::StockItem>**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_destroy

> stock_destroy(id)


API detail endpoint for a single StockItem instance.

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


## stock_install_create

> models::InstallStockItem stock_install_create(id, install_stock_item)


API endpoint for installing a particular stock item into this stock item.  - stock_item.part must be in the BOM for this part - stock_item must currently be \"in stock\" - stock_item must be serialized (and not belong to another item)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**install_stock_item** | [**InstallStockItem**](InstallStockItem.md) |  | [required] |

### Return type

[**models::InstallStockItem**](InstallStockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_list

> models::PaginatedStockItemList stock_list(limit, ipn, ipn_contains, ipn_regex, active, allocated, ancestor, assembly, available, batch, batch_regex, belongs_to, bom_item, build, cascade, category, company, consumed, consumed_by, customer, depleted, exclude_tree, expired, expiry_after, expiry_before, external, has_batch, has_child_items, has_installed_items, has_purchase_price, in_stock, include_variants, installed, is_building, location, location_detail, manufacturer, manufacturer_part, max_stock, min_stock, name, name_contains, name_regex, offset, ordering, part, part_detail, part_tree, path_detail, purchase_order, salable, sales_order, search, sent_to_customer, serial, serial_gte, serial_lte, serialized, stale, status, stocktake_after, stocktake_before, supplier, supplier_part, supplier_part_detail, tags__name, tags__slug, tests, tracked, updated_after, updated_before)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**ipn** | Option<**String**> | Part IPN (case insensitive) |  |
**ipn_contains** | Option<**String**> | Part IPN contains (case insensitive) |  |
**ipn_regex** | Option<**String**> | Part IPN (regex) |  |
**active** | Option<**bool**> | Active |  |
**allocated** | Option<**bool**> | Is Allocated |  |
**ancestor** | Option<**i32**> |  |  |
**assembly** | Option<**bool**> | Assembly |  |
**available** | Option<**bool**> | Available |  |
**batch** | Option<**String**> | Batch code filter (case insensitive) |  |
**batch_regex** | Option<**String**> | Batch code filter (regex) |  |
**belongs_to** | Option<**i32**> |  |  |
**bom_item** | Option<**i32**> |  |  |
**build** | Option<**i32**> |  |  |
**cascade** | Option<**bool**> | If true, include items in child locations of the given location |  |
**category** | Option<**i32**> |  |  |
**company** | Option<**i32**> |  |  |
**consumed** | Option<**bool**> | Consumed by Build Order |  |
**consumed_by** | Option<**i32**> |  |  |
**customer** | Option<**i32**> |  |  |
**depleted** | Option<**bool**> | Depleted |  |
**exclude_tree** | Option<**f64**> | Provide a StockItem PK to exclude that item and all its descendants |  |
**expired** | Option<**bool**> | Expired |  |
**expiry_after** | Option<**String**> | Expiry date after |  |
**expiry_before** | Option<**String**> | Expiry date before |  |
**external** | Option<**bool**> | External Location |  |
**has_batch** | Option<**bool**> | Has batch code |  |
**has_child_items** | Option<**bool**> | Has child items |  |
**has_installed_items** | Option<**bool**> | Has installed items |  |
**has_purchase_price** | Option<**bool**> | Has purchase price |  |
**in_stock** | Option<**bool**> | In Stock |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**installed** | Option<**bool**> | Installed in other stock item |  |
**is_building** | Option<**bool**> | In production |  |
**location** | Option<**i32**> | Filter by numeric Location ID or the literal 'null' |  |
**location_detail** | Option<**bool**> | Include detailed information about the stock location in the response |  |[default to false]
**manufacturer** | Option<**i32**> |  |  |
**manufacturer_part** | Option<**i32**> | Manufacturer Part |  |
**max_stock** | Option<**f64**> | Maximum stock |  |
**min_stock** | Option<**f64**> | Minimum stock |  |
**name** | Option<**String**> | Part name (case insensitive) |  |
**name_contains** | Option<**String**> | Part name contains (case insensitive) |  |
**name_regex** | Option<**String**> | Part name (regex) |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> | Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to true]
**part_tree** | Option<**i32**> |  |  |
**path_detail** | Option<**bool**> |  |  |[default to false]
**purchase_order** | Option<**i32**> |  |  |
**salable** | Option<**bool**> | Salable |  |
**sales_order** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: batch, location__name, part__IPN, part__description, part__name, serial, supplier_part__SKU, supplier_part__manufacturer_part__MPN, supplier_part__manufacturer_part__manufacturer__name, supplier_part__supplier__name, tags__name, tags__slug. |  |
**sent_to_customer** | Option<**bool**> | Sent to customer |  |
**serial** | Option<**String**> | Serial number |  |
**serial_gte** | Option<**i32**> | Serial number GTE |  |
**serial_lte** | Option<**i32**> | Serial number LTE |  |
**serialized** | Option<**bool**> | Has serial number |  |
**stale** | Option<**bool**> | Stale |  |
**status** | Option<**i32**> | Status Code |  |
**stocktake_after** | Option<**String**> | Stocktake After |  |
**stocktake_before** | Option<**String**> | Stocktake Before |  |
**supplier** | Option<**i32**> | Supplier |  |
**supplier_part** | Option<**i32**> |  |  |
**supplier_part_detail** | Option<**bool**> |  |  |[default to false]
**tags__name** | Option<**String**> |  |  |
**tags__slug** | Option<**String**> |  |  |
**tests** | Option<**bool**> |  |  |[default to false]
**tracked** | Option<**bool**> | Tracked |  |
**updated_after** | Option<**String**> | Updated after |  |
**updated_before** | Option<**String**> | Updated before |  |

### Return type

[**models::PaginatedStockItemList**](PaginatedStockItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_bulk_partial_update

> models::Location stock_location_bulk_partial_update(patched_location)


Perform a PATCH operation against this list endpoint.  Note that the typical DRF list endpoint does not support PATCH, so this method is provided as a custom implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_location** | Option<[**PatchedLocation**](PatchedLocation.md)> |  |  |

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_bulk_update

> models::Location stock_location_bulk_update(location)


Perform a PUT operation against this list endpoint.  Simply redirects to the PATCH method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location** | [**Location**](Location.md) |  | [required] |

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_create

> models::Location stock_location_create(location)


API endpoint for list view of StockLocation objects.  - GET: Return list of StockLocation objects - POST: Create a new StockLocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location** | [**Location**](Location.md) |  | [required] |

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_destroy

> stock_location_destroy(id)


Custom delete method to pass kwargs.

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


## stock_location_list

> models::PaginatedLocationList stock_location_list(limit, cascade, depth, external, has_location_type, location_type, name, offset, ordering, parent, path_detail, search, structural, top_level)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**cascade** | Option<**bool**> | Include sub-locations in filtered results |  |
**depth** | Option<**f64**> | Filter by location depth |  |
**external** | Option<**bool**> |  |  |
**has_location_type** | Option<**bool**> | has_location_type |  |
**location_type** | Option<**i32**> |  |  |
**name** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**parent** | Option<**i32**> | Filter by parent location |  |
**path_detail** | Option<**bool**> | Include detailed information about the BOM item linked to this build line. |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: description, name, pathstring, tags__name, tags__slug. |  |
**structural** | Option<**bool**> |  |  |
**top_level** | Option<**bool**> | Filter by top-level locations |  |

### Return type

[**models::PaginatedLocationList**](PaginatedLocationList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_partial_update

> models::Location stock_location_partial_update(id, patched_location)


Custom patch method to pass kwargs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_location** | Option<[**PatchedLocation**](PatchedLocation.md)> |  |  |

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_retrieve

> models::Location stock_location_retrieve(id, path_detail)


Custom get method to pass kwargs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**path_detail** | Option<**bool**> | Include detailed information about the BOM item linked to this build line. |  |[default to false]

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_tree_list

> models::PaginatedLocationTreeList stock_location_tree_list(limit, offset, ordering)


API endpoint for accessing a list of StockLocation objects, ready for rendering as a tree.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |

### Return type

[**models::PaginatedLocationTreeList**](PaginatedLocationTreeList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_type_create

> models::StockLocationType stock_location_type_create(stock_location_type)


API endpoint for a list of StockLocationType objects.  - GET: Return a list of all StockLocationType objects - POST: Create a StockLocationType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_location_type** | [**StockLocationType**](StockLocationType.md) |  | [required] |

### Return type

[**models::StockLocationType**](StockLocationType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_type_destroy

> stock_location_type_destroy(id)


API detail endpoint for a StockLocationType object.  - GET: return a single StockLocationType - PUT: update a StockLocationType - PATCH: partial update a StockLocationType - DELETE: delete a StockLocationType

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


## stock_location_type_list

> models::PaginatedStockLocationTypeList stock_location_type_list(limit, offset, ordering, search)


API endpoint for a list of StockLocationType objects.  - GET: Return a list of all StockLocationType objects - POST: Create a StockLocationType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: name. |  |

### Return type

[**models::PaginatedStockLocationTypeList**](PaginatedStockLocationTypeList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_type_partial_update

> models::StockLocationType stock_location_type_partial_update(id, patched_stock_location_type)


API detail endpoint for a StockLocationType object.  - GET: return a single StockLocationType - PUT: update a StockLocationType - PATCH: partial update a StockLocationType - DELETE: delete a StockLocationType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_stock_location_type** | Option<[**PatchedStockLocationType**](PatchedStockLocationType.md)> |  |  |

### Return type

[**models::StockLocationType**](StockLocationType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_type_retrieve

> models::StockLocationType stock_location_type_retrieve(id)


API detail endpoint for a StockLocationType object.  - GET: return a single StockLocationType - PUT: update a StockLocationType - PATCH: partial update a StockLocationType - DELETE: delete a StockLocationType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::StockLocationType**](StockLocationType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_type_update

> models::StockLocationType stock_location_type_update(id, stock_location_type)


API detail endpoint for a StockLocationType object.  - GET: return a single StockLocationType - PUT: update a StockLocationType - PATCH: partial update a StockLocationType - DELETE: delete a StockLocationType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**stock_location_type** | [**StockLocationType**](StockLocationType.md) |  | [required] |

### Return type

[**models::StockLocationType**](StockLocationType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_location_update

> models::Location stock_location_update(id, location)


Custom put method to pass kwargs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**location** | [**Location**](Location.md) |  | [required] |

### Return type

[**models::Location**](Location.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_merge_create

> models::StockMerge stock_merge_create(stock_merge)


API endpoint for merging multiple stock items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_merge** | [**StockMerge**](StockMerge.md) |  | [required] |

### Return type

[**models::StockMerge**](StockMerge.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_partial_update

> models::StockItem stock_partial_update(id, patched_stock_item)


API detail endpoint for a single StockItem instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_stock_item** | Option<[**PatchedStockItem**](PatchedStockItem.md)> |  |  |

### Return type

[**models::StockItem**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_remove_create

> models::StockRemove stock_remove_create(stock_remove)


Endpoint for removing a quantity of stock from an existing StockItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_remove** | [**StockRemove**](StockRemove.md) |  | [required] |

### Return type

[**models::StockRemove**](StockRemove.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_retrieve

> models::StockItem stock_retrieve(id, location_detail, part_detail, path_detail, supplier_part_detail, tests)


API detail endpoint for a single StockItem instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**location_detail** | Option<**bool**> | Include detailed information about the stock location in the response |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to true]
**path_detail** | Option<**bool**> |  |  |[default to false]
**supplier_part_detail** | Option<**bool**> |  |  |[default to false]
**tests** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::StockItem**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_return_create

> models::StockReturn stock_return_create(stock_return)


API endpoint for returning items into stock.  This API endpoint is for items that are initially considered \"not in stock\", and the user wants to return them to stock, marking them as \"available\" for further consumption or sale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_return** | [**StockReturn**](StockReturn.md) |  | [required] |

### Return type

[**models::StockReturn**](StockReturn.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_serial_numbers_retrieve

> models::StockItemSerialNumbers stock_serial_numbers_retrieve(id)


View extra serial number information for a given stock item.  Provides information on the \"previous\" and \"next\" stock items, based on the serial number of the given stock item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::StockItemSerialNumbers**](StockItemSerialNumbers.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_serialize_create

> Vec<models::StockItem> stock_serialize_create(id, serialize_stock_item)


API endpoint for serializing a stock item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**serialize_stock_item** | [**SerializeStockItem**](SerializeStockItem.md) |  | [required] |

### Return type

[**Vec<models::StockItem>**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_status_retrieve

> models::GenericStateClass stock_status_retrieve()


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


## stock_test_bulk_destroy

> stock_test_bulk_destroy(bulk_request)


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


## stock_test_create

> models::StockItemTestResult stock_test_create(stock_item_test_result)


API endpoint for listing (and creating) a StockItemTestResult object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_item_test_result** | [**StockItemTestResult**](StockItemTestResult.md) |  | [required] |

### Return type

[**models::StockItemTestResult**](StockItemTestResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_test_destroy

> stock_test_destroy(id)


Detail endpoint for StockItemTestResult.

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


## stock_test_list

> models::PaginatedStockItemTestResultList stock_test_list(limit, build, enabled, include_installed, offset, ordering, part, required, result, search, stock_item, template, template_detail, test, user, user_detail, value)


API endpoint for listing (and creating) a StockItemTestResult object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**build** | Option<**i32**> | Build |  |
**enabled** | Option<**bool**> | Enabled |  |
**include_installed** | Option<**bool**> | If true, include test results for items installed underneath the given stock item |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> | Part |  |
**required** | Option<**bool**> | Required |  |
**result** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. |  |
**stock_item** | Option<**i32**> | Filter by numeric Stock Item ID |  |
**template** | Option<**i32**> |  |  |
**template_detail** | Option<**bool**> |  |  |[default to false]
**test** | Option<**String**> | Test name (case insensitive) |  |
**user** | Option<**i32**> |  |  |
**user_detail** | Option<**bool**> |  |  |[default to false]
**value** | Option<**String**> |  |  |

### Return type

[**models::PaginatedStockItemTestResultList**](PaginatedStockItemTestResultList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_test_partial_update

> models::StockItemTestResult stock_test_partial_update(id, patched_stock_item_test_result)


Detail endpoint for StockItemTestResult.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_stock_item_test_result** | Option<[**PatchedStockItemTestResult**](PatchedStockItemTestResult.md)> |  |  |

### Return type

[**models::StockItemTestResult**](StockItemTestResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_test_retrieve

> models::StockItemTestResult stock_test_retrieve(id, template_detail, user_detail)


Detail endpoint for StockItemTestResult.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**template_detail** | Option<**bool**> |  |  |[default to false]
**user_detail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::StockItemTestResult**](StockItemTestResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_test_update

> models::StockItemTestResult stock_test_update(id, stock_item_test_result)


Detail endpoint for StockItemTestResult.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**stock_item_test_result** | [**StockItemTestResult**](StockItemTestResult.md) |  | [required] |

### Return type

[**models::StockItemTestResult**](StockItemTestResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_track_list

> models::PaginatedStockTrackingList stock_track_list(limit, include_variants, item, item_detail, max_date, min_date, offset, ordering, part, search, user, user_detail)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**include_variants** | Option<**bool**> | Include Part Variants |  |
**item** | Option<**i32**> |  |  |
**item_detail** | Option<**bool**> | Include detailed information about the item in the response |  |[default to false]
**max_date** | Option<**String**> | Date before |  |
**min_date** | Option<**String**> | Date after |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> | Part |  |
**search** | Option<**String**> | A search term. Searched fields: notes. |  |
**user** | Option<**i32**> |  |  |
**user_detail** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::PaginatedStockTrackingList**](PaginatedStockTrackingList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_track_retrieve

> models::StockTracking stock_track_retrieve(id)


Detail API endpoint for StockItemTracking model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::StockTracking**](StockTracking.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_track_status_retrieve

> models::GenericStateClass stock_track_status_retrieve()


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


## stock_transfer_create

> models::StockTransfer stock_transfer_create(stock_transfer)


API endpoint for performing stock movements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_transfer** | [**StockTransfer**](StockTransfer.md) |  | [required] |

### Return type

[**models::StockTransfer**](StockTransfer.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_uninstall_create

> models::UninstallStockItem stock_uninstall_create(id, uninstall_stock_item)


API endpoint for removing (uninstalling) items from this item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**uninstall_stock_item** | [**UninstallStockItem**](UninstallStockItem.md) |  | [required] |

### Return type

[**models::UninstallStockItem**](UninstallStockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stock_update

> models::StockItem stock_update(id, stock_item)


API detail endpoint for a single StockItem instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**stock_item** | [**StockItem**](StockItem.md) |  | [required] |

### Return type

[**models::StockItem**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

