<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Postman Request(2)</name>
   <tag></tag>
   <elementGuidId>6f1b6e98-f912-4fc9-9d51-4a573e497b19</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;name\&quot;: \&quot;Ismail-${$randomLastName}\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${InstitutionID}\&quot;,\r\n  \&quot;email\&quot;: \&quot;eniola@test.com\&quot;\r\n}&quot;,
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
      <id>a0101597-f5e9-43fe-a9b8-1aa17dd9156a</id>
      <masked>false</masked>
      <name>$randomLastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InstitutionID</defaultValue>
      <description></description>
      <id>15d1d6e7-4fc7-4975-b6b1-b181739dbc90</id>
      <masked>false</masked>
      <name>InstitutionID</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
