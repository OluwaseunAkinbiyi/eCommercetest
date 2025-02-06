<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>405c32cf-38a8-43e2-97cb-78982fbb4964</elementGuidId>
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
      <webElementGuid>f9ff1202-1c3d-4ab5-916e-af09013d07a7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:RunTransactionAsync2>
         &lt;!--Optional:-->
         &lt;tem:amount>850&lt;/tem:amount>
         &lt;!--Optional:-->
         &lt;tem:appzoneAccount>1100001111&lt;/tem:appzoneAccount>
         &lt;!--Optional:-->
         &lt;tem:institutionCode>100010&lt;/tem:institutionCode>
         &lt;!--Optional:-->
         &lt;tem:payer>John Doe&lt;/tem:payer>
         &lt;!--Optional:-->
         &lt;tem:payerAccountNumber>1100001561&lt;/tem:payerAccountNumber>
         &lt;!--Optional:-->
         &lt;tem:receiverAccountNumber>2091461522&lt;/tem:receiverAccountNumber>
         &lt;!--Optional:-->
         &lt;tem:receiverAccountType>savings&lt;/tem:receiverAccountType>
         &lt;!--Optional:-->
         &lt;tem:receiverBankCode>033&lt;/tem:receiverBankCode>
         &lt;!--Optional:-->
         &lt;tem:receiverPhnNo>08099277662&lt;/tem:receiverPhnNo>
         &lt;!--Optional:-->
         &lt;tem:receiverName>Jane Doe&lt;/tem:receiverName>
         &lt;!--Optional:-->
         &lt;tem:transactionReference>A08069833492&lt;/tem:transactionReference>
         &lt;!--Optional:-->
         &lt;tem:NIPSessionID>0000987093471237891234&lt;/tem:NIPSessionID>
         &lt;!--Optional:-->
         &lt;tem:IsDirectGateWayBilling>false&lt;/tem:IsDirectGateWayBilling>
         &lt;!--Optional:-->
         &lt;tem:narration>NEWBVNEAZYPAYBVN1&lt;/tem:narration>
         &lt;!--Optional:-->
         &lt;tem:receiverBVN>1122334455&lt;/tem:receiverBVN>
         &lt;!--Optional:-->
         &lt;tem:receiverKYC>112233445566&lt;/tem:receiverKYC>
      &lt;/tem:RunTransactionAsync2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.1.4.13/BankOneAgent/Services/TransferService.svc</soapServiceEndpoint>
   <soapServiceFunction>TransactionConfirmation</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>http://10.1.4.13/BankOneAgent/Services/TransferService.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
