[IP Authorization](/ipauthorization)

What's my IP?

# What's my IP

This endpoint lets you get your public IP address. It may be useful for IP Authorization purposes.

### Request & Response
    
    
    GET https://proxy.webshare.io/api/v2/proxy/ipauthorization/whatsmyip/

PythonJavascriptcURL

whatsmyip.py
    
    
    import requests
     
    response = requests.get(
        "https://proxy.webshare.io/api/v2/proxy/ipauthorization/whatsmyip/",
        headers={"Authorization": "Token APIKEY"}
    )
     
    response.json()

The commands above return JSON structured like this:

response.json
    
    
    { "ip_address": "1.2.3.4" }

[Delete IP authorization](/ipauthorization/delete "Delete IP authorization")[Proxy Statistics](/proxystats "Proxy Statistics")
