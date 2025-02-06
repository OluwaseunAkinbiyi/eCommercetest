<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Postman Request(1)</name>
   <tag></tag>
   <elementGuidId>dca6f186-9675-4a49-8e51-ec8d0492e6b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;name\&quot;: \&quot;Ismail\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${institutionId}\&quot;,\r\n  \&quot;email\&quot;: \&quot;user-${$randomEmail}\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bankonedatareferencing.azurewebsites.net/api/Auth/RegisterClient</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.institutionId</defaultValue>
      <description></description>
      <id>406f07bf-39ca-4d60-ab73-ea5aba953d03</id>
      <masked>false</masked>
      <name>institutionId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.$randomEmail</defaultValue>
      <description></description>
      <id>0bb8a60c-b0ee-4e2b-b70e-4bf03b1b103f</id>
      <masked>false</masked>
      <name>$randomEmail</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
