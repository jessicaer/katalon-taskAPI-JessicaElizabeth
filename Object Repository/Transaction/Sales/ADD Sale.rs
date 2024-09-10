<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ADD Sale</name>
   <tag></tag>
   <elementGuidId>60c69e96-a0f3-4f9b-93ca-b964b1196c8a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${accessToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;officeId\&quot;: \&quot;${officeID}\&quot;,\n    \&quot;customerId\&quot;: \&quot;${customerID}\&quot;,\n    \&quot;date\&quot;: \&quot;10/09/2024\&quot;,\n    \&quot;invoice\&quot;: \&quot;test\&quot;,\n    \&quot;amount\&quot;: 2000,\n    \&quot;discount\&quot;: 0,\n    \&quot;description\&quot;: \&quot;test saja\&quot;,\n    \&quot;items\&quot; : [\n        {\n            \&quot;productId\&quot;: \&quot;${productID}\&quot;,\n            \&quot;quantity\&quot;: 1,\n            \&quot;price\&quot;: 2000\n        }\n    ]\n}&quot;,
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
      <webElementGuid>3be51882-611a-484e-9092-9a6e4524266f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${accessToken}</value>
      <webElementGuid>d0a1c9d4-9e75-472a-877e-039472efdd1b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/sales</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>ce42ae9a-ce06-469b-b9e1-c9d26a4835d2</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessToken</defaultValue>
      <description></description>
      <id>18101c94-d02a-4263-a496-74b4e10e4252</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.officeID</defaultValue>
      <description></description>
      <id>5de9a1c7-bfab-46cb-a1c6-a90bbf354bca</id>
      <masked>false</masked>
      <name>officeID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customerID</defaultValue>
      <description></description>
      <id>c9a22b1f-27e4-4ff0-8471-965911e07e78</id>
      <masked>false</masked>
      <name>customerID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productID</defaultValue>
      <description></description>
      <id>13924a13-1a1c-4cfd-917e-0e788444de8b</id>
      <masked>false</masked>
      <name>productID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
