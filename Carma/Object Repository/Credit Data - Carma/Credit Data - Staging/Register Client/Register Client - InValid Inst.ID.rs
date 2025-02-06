<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register Client - InValid Inst.ID</name>
   <tag></tag>
   <elementGuidId>3b398697-edd2-42d1-bc6a-1537eeb667c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;name\&quot;: \&quot;Ismail-${$randomLastName}\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${InstitutionID}K\&quot;,\r\n  \&quot;email\&quot;: \&quot;user-${$randomEmail}\&quot;\r\n}&quot;,
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
      <id>dbde8e33-e3d8-4d01-b84d-c9ccc57bd29f</id>
      <masked>false</masked>
      <name>$randomLastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InstitutionID</defaultValue>
      <description></description>
      <id>b1d2b013-c210-41c2-9edb-8a8c541f8f5b</id>
      <masked>false</masked>
      <name>InstitutionID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.$randomEmail</defaultValue>
      <description></description>
      <id>a891b9e9-7658-4fac-a026-2e703261b9cf</id>
      <masked>false</masked>
      <name>$randomEmail</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
