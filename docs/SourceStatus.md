# SourceStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | The current status of the source crawl for this collection. This field returns `not_configured` if the default configuration for this source does not have a **source** object defined.  -  `running` indicates that a crawl to fetch more documents is in progress. -  `complete` indicates that the crawl has completed with no errors. -  `complete_with_notices` indicates that some notices were generated during the crawl. Notices can be checked by using the **notices** query method. -  `stopped` indicates that the crawl has stopped but is not complete. | [optional] 
**last_updated** | **String** | Date in UTC format indicating when the last crawl was attempted. If `null`, no crawl was completed. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


