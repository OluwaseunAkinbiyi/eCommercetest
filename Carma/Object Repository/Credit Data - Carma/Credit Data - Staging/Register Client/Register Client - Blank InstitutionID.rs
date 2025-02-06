<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register Client - Blank InstitutionID</name>
   <tag></tag>
   <elementGuidId>c15290fb-0ede-47a1-a7a4-85deabb387ed</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\r\n  \&quot;name\&quot;: \&quot;Ismail-${$randomLastName}\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;\&quot;,\r\n  \&quot;email\&quot;: \&quot;user-${$randomEmail}\&quot;\r\n}\r\n&quot;,
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
      <defaultValue>GlobalVariable.$randomLastName</defaultValue>
      <description></description>
      <id>b01e4b57-dfa5-4ce9-9065-4efab45ddc28</id>
      <masked>false</masked>
      <name>$randomLastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.$randomEmail</defaultValue>
      <description></description>
      <id>d796c704-2c12-4044-9456-4cea54851c0a</id>
      <masked>false</masked>
      <name>$randomEmail</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
