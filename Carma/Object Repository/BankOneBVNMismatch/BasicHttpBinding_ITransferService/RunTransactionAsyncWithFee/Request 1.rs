<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>83d62ff6-e300-4d57-acd3-db7d419f2fba</elementGuidId>
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
      <webElementGuid>21f637a1-f810-421f-9a6f-cf11f57fb46f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:RunTransactionAsyncWithFee>
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
         &lt;!--Optional:-->
         &lt;tem:receiverBVN>?&lt;/tem:receiverBVN>
         &lt;!--Optional:-->
         &lt;tem:receiverKYC>?&lt;/tem:receiverKYC>
         &lt;!--Optional:-->
         &lt;tem:thirdpartyFee>?&lt;/tem:thirdpartyFee>
         &lt;!--Optional:-->
         &lt;tem:useThirdPartyFee>?&lt;/tem:useThirdPartyFee>
         &lt;!--Optional:-->
         &lt;tem:debitExpenseForFee>?&lt;/tem:debitExpenseForFee>
         &lt;!--Optional:-->
         &lt;tem:secondaryInstitutionCode>?&lt;/tem:secondaryInstitutionCode>
         &lt;!--Optional:-->
         &lt;tem:timeInitiated>?&lt;/tem:timeInitiated>
      &lt;/tem:RunTransactionAsyncWithFee>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>RunTransactionAsyncWithFee</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
