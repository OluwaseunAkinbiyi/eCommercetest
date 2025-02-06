<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>b6a5a72d-ae56-453c-94cb-4678d4e5a0f9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>bd39aaf1-a923-4677-b971-1608e2e24bf8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:GetFee>
         &lt;!--Optional:-->
         &lt;tem:institutionCode>?&lt;/tem:institutionCode>
         &lt;!--Optional:-->
         &lt;tem:amount>?&lt;/tem:amount>
      &lt;/tem:GetFee>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>GetFee</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
