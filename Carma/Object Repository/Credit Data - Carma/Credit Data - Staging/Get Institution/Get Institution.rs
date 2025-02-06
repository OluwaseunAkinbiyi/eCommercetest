<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Institution</name>
   <tag></tag>
   <elementGuidId>53f2a699-ab7d-493c-9efd-13b8c3749e2c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJlYWdiYWplQHFvcmUuaW5jIiwianRpIjoiM2Y3NjYxZmEtYmY2OC00N2JiLTk2NGItNDgwMjM5YWY2YmMxIiwiZW1haWwiOiJlYWdiYWplQHFvcmUuaW5jIiwiaWQiOiI2NzkyMTVhNjgzNmQ0NTNkZmNkMDcxMDQiLCJpbnN0aXR1dGlvbklkIjoiIiwicm9sZSI6InN1cGVyYWRtaW4iLCJuYmYiOjE3MzgxODE0NzEsImV4cCI6MTczODE5NTE5OSwiaWF0IjoxNzM4MTgxNDcxfQ.rQ34SoCTNrH4CxzRkxhrPyf4bXDMJ11QdHotmlRgSWY</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJlYWdiYWplQHFvcmUuaW5jIiwianRpIjoiM2Y3NjYxZmEtYmY2OC00N2JiLTk2NGItNDgwMjM5YWY2YmMxIiwiZW1haWwiOiJlYWdiYWplQHFvcmUuaW5jIiwiaWQiOiI2NzkyMTVhNjgzNmQ0NTNkZmNkMDcxMDQiLCJpbnN0aXR1dGlvbklkIjoiIiwicm9sZSI6InN1cGVyYWRtaW4iLCJuYmYiOjE3MzgxODE0NzEsImV4cCI6MTczODE5NTE5OSwiaWF0IjoxNzM4MTgxNDcxfQ.rQ34SoCTNrH4CxzRkxhrPyf4bXDMJ11QdHotmlRgSWY</value>
      <webElementGuid>3426323e-17c2-4e11-94ca-a01f6ce5b5c1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://bankonedatareferencing-staging-ddcsfuf9cxgydnac.eastus-01.azurewebsites.net/api/Institution</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
