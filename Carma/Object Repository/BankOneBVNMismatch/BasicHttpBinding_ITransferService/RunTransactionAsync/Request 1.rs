<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>a6f9e67b-2a26-49ef-83a3-164156b9020f</elementGuidId>
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
      <webElementGuid>73c2d133-cf3d-4654-a711-a1887c904f19</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:RunTransactionAsync>
         &lt;!--Optional:-->
         &lt;tem:amount>?&lt;/tem:amount>
         &lt;!--Optional:-->
         &lt;tem:appzoneAccount>?&lt;/tem:appzoneAccount>
         &lt;!--Optional:-->
         &lt;tem:institutionCode>?&lt;/tem:institutionCode>
         &lt;!--Optional:-->
         &lt;tem:payer>?&lt;/tem:payer>
         &lt;!--Optional:-->
         &lt;tem:payerAccountNumber>?&lt;/tem:payerAccountNumber>
         &lt;!--Optional:-->
         &lt;tem:receiverAccountNumber>?&lt;/tem:receiverAccountNumber>
         &lt;!--Optional:-->
         &lt;tem:receiverAccountType>?&lt;/tem:receiverAccountType>
         &lt;!--Optional:-->
         &lt;tem:receiverBankCode>?&lt;/tem:receiverBankCode>
         &lt;!--Optional:-->
         &lt;tem:receiverPhnNo>?&lt;/tem:receiverPhnNo>
         &lt;!--Optional:-->
         &lt;tem:receiverName>?&lt;/tem:receiverName>
         &lt;!--Optional:-->
         &lt;tem:transactionReference>?&lt;/tem:transactionReference>
         &lt;!--Optional:-->
         &lt;tem:NIPSessionID>?&lt;/tem:NIPSessionID>
         &lt;!--Optional:-->
         &lt;tem:IsDirectGateWayBilling>?&lt;/tem:IsDirectGateWayBilling>
         &lt;!--Optional:-->
         &lt;tem:narration>?&lt;/tem:narration>
      &lt;/tem:RunTransactionAsync>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>RunTransactionAsync</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
