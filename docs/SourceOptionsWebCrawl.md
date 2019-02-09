# SourceOptionsWebCrawl

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The starting URL to crawl. | 
**limit_to_starting_hosts** | **bool** | When `true`, crawls of the specified URL are limited to the host part of the **url** field. | [optional] [default to true]
**crawl_speed** | **String** | The number of concurrent URLs to fetch. `gentle` means one URL is fetched at a time with a delay between each call. `normal` means as many as two URLs are fectched concurrently with a short delay between fetch calls. `aggressive` means that up to ten URLs are fetched concurrently with a short delay between fetch calls. | [optional] [default to normal]
**allow_untrusted_certificate** | **bool** | When `true`, allows the crawl to interact with HTTPS sites with SSL certificates with untrusted signers. | [optional] [default to false]
**maximum_hops** | **i32** | The maximum number of hops to make from the initial URL. When a page is crawled each link on that page will also be crawled if it is within the **maximum_hops** from the initial URL. The first page crawled is 0 hops, each link crawled from the first page is 1 hop, each link crawled from those pages is 2 hops, and so on. | [optional] 
**request_timeout** | **i32** | The maximum milliseconds to wait for a response from the web server. | [optional] 
**override_robots_txt** | **bool** | When `true`, the crawler will ignore any `robots.txt` encountered by the crawler. This should only ever be done when crawling a web site the user owns. This must be be set to `true` when a **gateway_id** is specied in the **credentials**. | [optional] [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


