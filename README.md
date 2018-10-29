# Watson Discovery API

This is a rewrite and update of [wdsapi](https://github.com/bruceadams/wdsapi).
[![Build Status](https://www.travis-ci.com/bruceadams/watson-discovery-api.svg?branch=master)](https://www.travis-ci.com/bruceadams/watson-discovery-api)

## Overall structure

I'd like to be able to generate this code from the Swagger definition of the Watson Discovery API. I'm not there yet. The two other Rust targets for Swagger are too far away from from what I want to see.

I want to have a Rust type, a struct, for each kind of thing the API acts on. Things such as: _environment_, _configuration_ and _collection_.

## Authentication options

The authentication side of things is a little messy. The primary authentication mechanism should be an IAM apikey, while still supporting the older Cloud Foundry username and password mechanism.

The challenge with using an apikey, is the endpoint for getting a token. The global standard endpoint is https://iam.bluemix.net/identity/token. Unfortunately, that endpoint is not quite universal. There are regional endpoints, that might be more appropriate, for speed, reachability and maybe other things I'm not aware of. Worse, there is a different endpoint for IBM internal, pre-production systems. I want this library to support all of these possibilities, without being annoying. Most of the time, the standard endpoint will _just work_. I don't want to ask the API user to specify something that they shouldn't need to worry about.

Some approaches:

1. Optional user specified token endpoint.
    - Optional, um, how, exactly?
2. Embed knowledge of _all_ endpoints in the code and try them all concurrently and accept the first successful response.
    - Can I reasonably _know_ all of the token endpoints?
    - Maybe a deep, dark, option for the user to specifiy the list of endpoints to try. Erg.
