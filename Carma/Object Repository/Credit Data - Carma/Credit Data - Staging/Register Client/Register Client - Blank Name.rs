<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register Client - Blank Name</name>
   <tag></tag>
   <elementGuidId>c0b5358d-109e-40b3-aad6-1d6f0dce0262</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\r\n  \&quot;name\&quot;: \&quot;\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${InstitutionID}\&quot;,\r\n  \&quot;email\&quot;: \&quot;user-${$randomEmail}\&quot;\r\n}\r\n&quot;,
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
      <defaultValue>GlobalVariable.InstitutionID</defaultValue>
      <description></description>
      <id>65548a82-7e37-48ec-bc6b-3d24f12af8a2</id>
      <masked>false</masked>
      <name>InstitutionID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.$randomEmail</defaultValue>
      <description></description>
      <id>2f8d27a8-b996-45ac-80ac-a1b4c2c91208</id>
      <masked>false</masked>
      <name>$randomEmail</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
