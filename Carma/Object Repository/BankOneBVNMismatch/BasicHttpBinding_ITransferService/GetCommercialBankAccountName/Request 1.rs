<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>00acc445-f691-4380-b348-c00403a3f709</elementGuidId>
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
      <webElementGuid>128f6e61-1eae-4b57-a29d-64c03e5a024c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:GetCommercialBankAccountName>
         &lt;!--Optional:-->
         &lt;tem:mfbCode>100010&lt;/tem:mfbCode>
         &lt;!--Optional:-->
         &lt;tem:bankCode>503&lt;/tem:bankCode>
         &lt;!--Optional:-->
         &lt;tem:accountNumber>1290909090&lt;/tem:accountNumber>
      &lt;/tem:GetCommercialBankAccountName>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>GetCommercialBankAccountName</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
