# PatchedExtendedUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**username** | Option<**String**> | Username | [optional]
**first_name** | Option<**String**> | First name of the user | [optional]
**last_name** | Option<**String**> | Last name of the user | [optional]
**email** | Option<**String**> | Email address of the user | [optional]
**groups** | Option<[**Vec<models::Group>**](Group.md)> |  | [optional][readonly]
**group_ids** | Option<**Vec<i32>**> |  | [optional]
**is_staff** | Option<**bool**> | Does this user have staff permissions | [optional]
**is_superuser** | Option<**bool**> | Is this user a superuser | [optional]
**is_active** | Option<**bool**> | Is this user account active | [optional]
**profile** | Option<[**models::BriefUserProfile**](BriefUserProfile.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


