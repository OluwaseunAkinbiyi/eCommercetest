<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Admin - Valid</name>
   <tag></tag>
   <elementGuidId>98a8f8fe-050a-40e6-a900-abaabc7c45e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${authToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;${randomName}\&quot;,\n  \&quot;email\&quot;: \&quot;${randomEmail}\&quot;,\n  \&quot;password\&quot;: \&quot;password\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9d212fb9-189e-4d30-90b2-a1e90253ae49</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${authToken}</value>
      <webElementGuid>b6265cda-c566-4d69-92ad-36f540e55c6b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://bankonedatareferencing-staging-ddcsfuf9cxgydnac.eastus-01.azurewebsites.net/api/Auth/CreateAdmin</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJlYWdiYWplQHFvcmUuaW5jIiwianRpIjoiNTZhZTczMjItMzc2OS00OGFiLWFlZWItMTcyNjE4NjJkZDZjIiwiZW1haWwiOiJlYWdiYWplQHFvcmUuaW5jIiwiaWQiOiI2NzkyMTVhNjgzNmQ0NTNkZmNkMDcxMDQiLCJpbnN0aXR1dGlvbklkIjoiIiwicm9sZSI6InN1cGVyYWRtaW4iLCJuYmYiOjE3Mzg3OTUxNTYsImV4cCI6MTczODc5OTk5OSwiaWF0IjoxNzM4Nzk1MTU2fQ.s7it2-6OvqIj9XBsFrrmmSut7lJid4oKuoAbL8fopHY'</defaultValue>
      <description></description>
      <id>78fb7702-8f24-4ed0-8a06-698d3a8fa270</id>
      <masked>false</masked>
      <name>authToken</name>
   </variables>
   <variables>
      <defaultValue>'User${System.currentTimeMillis()}'</defaultValue>
      <description></description>
      <id>eac6fcf2-8141-4b2f-b998-97e8208afcb2</id>
      <masked>false</masked>
      <name>randomName</name>
   </variables>
   <variables>
      <defaultValue>'user${System.currentTimeMillis()}@test.com'</defaultValue>
      <description></description>
      <id>5f3052b4-a4ec-4f27-bf2f-99879c43bf74</id>
      <masked>false</masked>
      <name>randomEmail</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
