<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register Client - InValid email</name>
   <tag></tag>
   <elementGuidId>2756ed4e-d1de-48aa-935e-ceba7bb8da3f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;name\&quot;: \&quot;Ismail-${$randomLastName}\&quot;,\r\n  \&quot;institutionId\&quot;: \&quot;${InstitutionID}\&quot;,\r\n  \&quot;email\&quot;: \&quot;usersss-@test\&quot;\r\n}&quot;,
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
      <id>6a207f20-7fa7-4b93-b8a2-4a7efa0478a1</id>
      <masked>false</masked>
      <name>$randomLastName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InstitutionID</defaultValue>
      <description></description>
      <id>4b572c4d-385e-433a-84ca-bdee7cc668d3</id>
      <masked>false</masked>
      <name>InstitutionID</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
