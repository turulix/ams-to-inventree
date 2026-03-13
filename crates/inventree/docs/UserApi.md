# \UserApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_create**](UserApi.md#user_create) | **POST** /api/user/ | 
[**user_destroy**](UserApi.md#user_destroy) | **DELETE** /api/user/{id}/ | 
[**user_group_create**](UserApi.md#user_group_create) | **POST** /api/user/group/ | 
[**user_group_destroy**](UserApi.md#user_group_destroy) | **DELETE** /api/user/group/{id}/ | 
[**user_group_list**](UserApi.md#user_group_list) | **GET** /api/user/group/ | 
[**user_group_partial_update**](UserApi.md#user_group_partial_update) | **PATCH** /api/user/group/{id}/ | 
[**user_group_retrieve**](UserApi.md#user_group_retrieve) | **GET** /api/user/group/{id}/ | 
[**user_group_update**](UserApi.md#user_group_update) | **PUT** /api/user/group/{id}/ | 
[**user_list**](UserApi.md#user_list) | **GET** /api/user/ | 
[**user_me_destroy**](UserApi.md#user_me_destroy) | **DELETE** /api/user/me/ | 
[**user_me_partial_update**](UserApi.md#user_me_partial_update) | **PATCH** /api/user/me/ | 
[**user_me_retrieve**](UserApi.md#user_me_retrieve) | **GET** /api/user/me/ | 
[**user_me_update**](UserApi.md#user_me_update) | **PUT** /api/user/me/ | 
[**user_owner_list**](UserApi.md#user_owner_list) | **GET** /api/user/owner/ | 
[**user_owner_retrieve**](UserApi.md#user_owner_retrieve) | **GET** /api/user/owner/{id}/ | 
[**user_partial_update**](UserApi.md#user_partial_update) | **PATCH** /api/user/{id}/ | 
[**user_profile_partial_update**](UserApi.md#user_profile_partial_update) | **PATCH** /api/user/profile/ | 
[**user_profile_retrieve**](UserApi.md#user_profile_retrieve) | **GET** /api/user/profile/ | 
[**user_profile_update**](UserApi.md#user_profile_update) | **PUT** /api/user/profile/ | 
[**user_retrieve**](UserApi.md#user_retrieve) | **GET** /api/user/{id}/ | 
[**user_roles_retrieve**](UserApi.md#user_roles_retrieve) | **GET** /api/user/roles/ | 
[**user_ruleset_destroy**](UserApi.md#user_ruleset_destroy) | **DELETE** /api/user/ruleset/{id}/ | 
[**user_ruleset_list**](UserApi.md#user_ruleset_list) | **GET** /api/user/ruleset/ | 
[**user_ruleset_partial_update**](UserApi.md#user_ruleset_partial_update) | **PATCH** /api/user/ruleset/{id}/ | 
[**user_ruleset_retrieve**](UserApi.md#user_ruleset_retrieve) | **GET** /api/user/ruleset/{id}/ | 
[**user_ruleset_update**](UserApi.md#user_ruleset_update) | **PUT** /api/user/ruleset/{id}/ | 
[**user_set_password_partial_update**](UserApi.md#user_set_password_partial_update) | **PATCH** /api/user/{id}/set-password/ | 
[**user_set_password_update**](UserApi.md#user_set_password_update) | **PUT** /api/user/{id}/set-password/ | 
[**user_token_retrieve**](UserApi.md#user_token_retrieve) | **GET** /api/user/token/ | 
[**user_tokens_create**](UserApi.md#user_tokens_create) | **POST** /api/user/tokens/ | 
[**user_tokens_destroy**](UserApi.md#user_tokens_destroy) | **DELETE** /api/user/tokens/{id}/ | 
[**user_tokens_list**](UserApi.md#user_tokens_list) | **GET** /api/user/tokens/ | 
[**user_tokens_retrieve**](UserApi.md#user_tokens_retrieve) | **GET** /api/user/tokens/{id}/ | 
[**user_update**](UserApi.md#user_update) | **PUT** /api/user/{id}/ | 



## user_create

> models::UserCreate user_create(user_create)


List endpoint for detail on all users.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create** | [**UserCreate**](UserCreate.md) |  | [required] |

### Return type

[**models::UserCreate**](UserCreate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_destroy

> user_destroy(id)


Detail endpoint for a single user.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

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


## user_group_create

> models::Group user_group_create(group)


List endpoint for all auth groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | [**Group**](Group.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_group_destroy

> user_group_destroy(id)


Detail endpoint for a particular auth group.

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


## user_group_list

> models::PaginatedGroupList user_group_list(limit, offset, ordering, permission_detail, role_detail, search, user_detail)


List endpoint for all auth groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**permission_detail** | Option<**bool**> | Include permission details |  |[default to false]
**role_detail** | Option<**bool**> | Include role details |  |[default to true]
**search** | Option<**String**> | A search term. Searched fields: name. |  |
**user_detail** | Option<**bool**> | Include user details |  |[default to false]

### Return type

[**models::PaginatedGroupList**](PaginatedGroupList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_group_partial_update

> models::Group user_group_partial_update(id, patched_group)


Detail endpoint for a particular auth group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_group** | Option<[**PatchedGroup**](PatchedGroup.md)> |  |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_group_retrieve

> models::Group user_group_retrieve(id, permission_detail, role_detail, user_detail)


Detail endpoint for a particular auth group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**permission_detail** | Option<**bool**> | Include permission details |  |[default to false]
**role_detail** | Option<**bool**> | Include role details |  |[default to true]
**user_detail** | Option<**bool**> | Include user details |  |[default to false]

### Return type

[**models::Group**](Group.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_group_update

> models::Group user_group_update(id, group)


Detail endpoint for a particular auth group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**group** | [**Group**](Group.md) |  | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list

> models::PaginatedUserCreateList user_list(limit, is_active, is_staff, is_superuser, offset, ordering, search)


List endpoint for detail on all users.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**is_active** | Option<**bool**> |  |  |
**is_staff** | Option<**bool**> |  |  |
**is_superuser** | Option<**bool**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: first_name, last_name, username. |  |

### Return type

[**models::PaginatedUserCreateList**](PaginatedUserCreateList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_me_destroy

> user_me_destroy()


Detail endpoint for current user.  Permissions: - User can edit their own details via this endpoint - Only a subset of fields are available here

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


## user_me_partial_update

> models::MeUser user_me_partial_update(patched_me_user)


Detail endpoint for current user.  Permissions: - User can edit their own details via this endpoint - Only a subset of fields are available here

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_me_user** | Option<[**PatchedMeUser**](PatchedMeUser.md)> |  |  |

### Return type

[**models::MeUser**](MeUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_me_retrieve

> models::MeUser user_me_retrieve()


Detail endpoint for current user.  Permissions: - User can edit their own details via this endpoint - Only a subset of fields are available here

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MeUser**](MeUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_me_update

> models::MeUser user_me_update(me_user)


Detail endpoint for current user.  Permissions: - User can edit their own details via this endpoint - Only a subset of fields are available here

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**me_user** | [**MeUser**](MeUser.md) |  | [required] |

### Return type

[**models::MeUser**](MeUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_owner_list

> models::PaginatedOwnerList user_owner_list(limit, is_active, offset, ordering, search)


List API endpoint for Owner model.  Cannot create a new Owner object via the API, but can view existing instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**is_active** | Option<**bool**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedOwnerList**](PaginatedOwnerList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_owner_retrieve

> models::Owner user_owner_retrieve(id)


Detail API endpoint for Owner model.  Cannot edit or delete

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Owner**](Owner.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_partial_update

> models::ExtendedUser user_partial_update(id, patched_extended_user)


Detail endpoint for a single user.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_extended_user** | Option<[**PatchedExtendedUser**](PatchedExtendedUser.md)> |  |  |

### Return type

[**models::ExtendedUser**](ExtendedUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profile_partial_update

> models::UserProfile user_profile_partial_update(patched_user_profile)


Detail endpoint for the user profile.  Permissions: - Any authenticated user has write access against this endpoint - The endpoint always returns the profile associated with the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_user_profile** | Option<[**PatchedUserProfile**](PatchedUserProfile.md)> |  |  |

### Return type

[**models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profile_retrieve

> models::UserProfile user_profile_retrieve()


Detail endpoint for the user profile.  Permissions: - Any authenticated user has write access against this endpoint - The endpoint always returns the profile associated with the current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profile_update

> models::UserProfile user_profile_update(user_profile)


Detail endpoint for the user profile.  Permissions: - Any authenticated user has write access against this endpoint - The endpoint always returns the profile associated with the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_profile** | Option<[**UserProfile**](UserProfile.md)> |  |  |

### Return type

[**models::UserProfile**](UserProfile.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_retrieve

> models::ExtendedUser user_retrieve(id)


Detail endpoint for a single user.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ExtendedUser**](ExtendedUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_roles_retrieve

> models::Role user_roles_retrieve()


API endpoint which lists the available role permissions for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Role**](Role.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_ruleset_destroy

> user_ruleset_destroy(id)


Detail endpoint for a particular RuleSet instance.

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


## user_ruleset_list

> models::PaginatedRuleSetList user_ruleset_list(limit, group, name, offset, ordering, search)


List endpoint for all RuleSet instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**group** | Option<**i32**> |  |  |
**name** | Option<**String**> | Permission set  * `admin` - Admin * `part_category` - Part Categories * `part` - Parts * `stock_location` - Stock Locations * `stock` - Stock Items * `build` - Build Orders * `purchase_order` - Purchase Orders * `sales_order` - Sales Orders * `return_order` - Return Orders |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: name. |  |

### Return type

[**models::PaginatedRuleSetList**](PaginatedRuleSetList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_ruleset_partial_update

> models::RuleSet user_ruleset_partial_update(id, patched_rule_set)


Detail endpoint for a particular RuleSet instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_rule_set** | Option<[**PatchedRuleSet**](PatchedRuleSet.md)> |  |  |

### Return type

[**models::RuleSet**](RuleSet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_ruleset_retrieve

> models::RuleSet user_ruleset_retrieve(id)


Detail endpoint for a particular RuleSet instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RuleSet**](RuleSet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_ruleset_update

> models::RuleSet user_ruleset_update(id, rule_set)


Detail endpoint for a particular RuleSet instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**rule_set** | Option<[**RuleSet**](RuleSet.md)> |  |  |

### Return type

[**models::RuleSet**](RuleSet.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_set_password_partial_update

> models::UserSetPassword user_set_password_partial_update(id, patched_user_set_password)


Allows superusers to set the password for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_user_set_password** | Option<[**PatchedUserSetPassword**](PatchedUserSetPassword.md)> |  |  |

### Return type

[**models::UserSetPassword**](UserSetPassword.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_set_password_update

> models::UserSetPassword user_set_password_update(id, user_set_password)


Allows superusers to set the password for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**user_set_password** | [**UserSetPassword**](UserSetPassword.md) |  | [required] |

### Return type

[**models::UserSetPassword**](UserSetPassword.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_token_retrieve

> models::GetAuthToken user_token_retrieve(name)


Return an API token if the user is authenticated.  - If the user already has a matching token, delete it and create a new one - Existing tokens are *never* exposed again via the API - Once the token is provided, it can be used for auth until it expires

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the token |  |

### Return type

[**models::GetAuthToken**](GetAuthToken.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_tokens_create

> models::ApiToken user_tokens_create(api_token)


List of user tokens for current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_token** | Option<[**ApiToken**](ApiToken.md)> |  |  |

### Return type

[**models::ApiToken**](ApiToken.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_tokens_destroy

> user_tokens_destroy(id)


Details for a user token.

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


## user_tokens_list

> models::PaginatedApiTokenList user_tokens_list(limit, offset, ordering, revoked, search, user)


List of user tokens for current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**revoked** | Option<**bool**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: key, name. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedApiTokenList**](PaginatedApiTokenList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_tokens_retrieve

> models::ApiToken user_tokens_retrieve(id, all_users)


Details for a user token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**all_users** | Option<**bool**> | Display tokens for all users (superuser only) |  |

### Return type

[**models::ApiToken**](ApiToken.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update

> models::ExtendedUser user_update(id, extended_user)


Detail endpoint for a single user.  Permissions: - Staff users (who also have the 'admin' role) can perform write operations - Otherwise authenticated users have read-only access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**extended_user** | [**ExtendedUser**](ExtendedUser.md) |  | [required] |

### Return type

[**models::ExtendedUser**](ExtendedUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

