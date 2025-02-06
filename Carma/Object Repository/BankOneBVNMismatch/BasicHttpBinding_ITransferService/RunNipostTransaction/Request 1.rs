<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>b2fff419-fbc9-4bc3-af7c-e125f3d1c673</elementGuidId>
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
      <webElementGuid>3f6a779a-7987-45eb-b59f-c86edca3764d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:RunNipostTransaction>
         &lt;!--Optional:-->
         &lt;tem:nipostCode>?&lt;/tem:nipostCode>
         &lt;!--Optional:-->
         &lt;tem:mfbCode>?&lt;/tem:mfbCode>
         &lt;!--Optional:-->
         &lt;tem:customerAccount>?&lt;/tem:customerAccount>
         &lt;!--Optional:-->
         &lt;tem:amount>?&lt;/tem:amount>
         &lt;!--Optional:-->
         &lt;tem:narration>?&lt;/tem:narration>
         &lt;!--Optional:-->
         &lt;tem:transactionReference>?&lt;/tem:transactionReference>
         &lt;!--Optional:-->
         &lt;tem:transactionType>?&lt;/tem:transactionType>
      &lt;/tem:RunNipostTransaction>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>RunNipostTransaction</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
