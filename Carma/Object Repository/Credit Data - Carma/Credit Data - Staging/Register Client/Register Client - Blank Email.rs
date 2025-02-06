<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register Client - Blank Email</name>
   <tag></tag>
   <elementGuidId>efba0683-17cb-40ad-9507-32bc4fe2c43e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n\r\n  \&quot;name\&quot;: \&quot;Ismail-${$randomLastName}\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${InstitutionID}\&quot;,\r\n  \&quot;email\&quot;: \&quot;\&quot;\r\n}\r\n&quot;,
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
      <id>a2b69044-9bea-4368-9663-9a95fd80f1d1</id>
      <masked>false</masked>
      <name>$randomLastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InstitutionID</defaultValue>
      <description></description>
      <id>be81a435-923b-4135-b71f-0793aafcf817</id>
      <masked>false</masked>
      <name>InstitutionID</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
