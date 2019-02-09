# SourceOptions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**folders** | [**Vec<::models::SourceOptionsFolder>**](SourceOptionsFolder.md) | Array of folders to crawl from the Box source. Only valid, and required, when the **type** field of the **source** object is set to `box`. | [optional] 
**objects** | [**Vec<::models::SourceOptionsObject>**](SourceOptionsObject.md) | Array of Salesforce document object types to crawl from the Salesforce source. Only valid, and required, when the **type** field of the **source** object is set to `salesforce`. | [optional] 
**site_collections** | [**Vec<::models::SourceOptionsSiteColl>**](SourceOptionsSiteColl.md) | Array of Microsoft SharePointoint Online site collections to crawl from the SharePoint source. Only valid and required when the **type** field of the **source** object is set to `sharepoint`. | [optional] 
**urls** | [**Vec<::models::SourceOptionsWebCrawl>**](SourceOptionsWebCrawl.md) | Array of Web page URLs to begin crawling the web from. Only valid and required when the **type** field of the **source** object is set to `web_crawl`. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


