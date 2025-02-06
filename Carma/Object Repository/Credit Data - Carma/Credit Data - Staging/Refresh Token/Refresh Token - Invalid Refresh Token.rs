<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Refresh Token - Invalid Refresh Token</name>
   <tag></tag>
   <elementGuidId>e69ffb3a-8e3e-463c-9166-b7fffd67a5fb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;token\&quot;: \&quot;eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLUpvaG5wYXVsMTJAeWFob28uY29tIiwianRpIjoiYTQ1NjMyYWQtODhkZi00YTQzLThlMTMtY2E0OTgwZWQ5MTQxIiwiZW1haWwiOiJ1c2VyLUpvaG5wYXVsMTJAeWFob28uY29tIiwiaWQiOiI2MzJjZDIwMGIxNTQxNTI3YWM2MjU1YmYiLCJpbnN0aXR1dGlvbklkIjoiNjMxY2U5ZWFlMTI0YWI2Njc4NjlmYjZiIiwicm9sZSI6Im9hdXRoY2xpZW50IiwibmJmIjoxNjYzODgzMjI0LCJleHAiOjE2NjM4OTExOTksImlhdCI6MTY2Mzg4MzIyNH0.k4ABrrDOeqGOK1NKlWVVwi6Mzgk5UQ5FUFkaWN7tKbU\&quot;,\n  \&quot;refreshToken\&quot;: \&quot;DdB8lvegCJcjy2dLo1p2zQEh3+Sy0velypkpuhFuvKPgnYogmBB52XNWg002nT9rJBp6A5wN3sZ1Y1/WkCe3G/g\u003d\u003d\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bankonedatareferencing-staging-ddcsfuf9cxgydnac.eastus-01.azurewebsites.net/api/Auth/RefreshToken</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 415)

assertThat(response.getStatusCode()).isEqualTo(415)


WS.verifyElementPropertyValue(response, 'message', 'Authentication failed')


WS.verifyElementPropertyValue(response, 'errors', '[Invalid Token]')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
