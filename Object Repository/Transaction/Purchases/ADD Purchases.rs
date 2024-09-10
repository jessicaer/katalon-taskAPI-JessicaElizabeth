<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ADD Purchases</name>
   <tag></tag>
   <elementGuidId>2ed177eb-21d1-429a-ba5c-937e3cecd39e</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;officeId\&quot;: \&quot;${officeID}\&quot;,\n    \&quot;date\&quot;: \&quot;10-09-2024\&quot;,\n    \&quot;invoice\&quot;: \&quot;INV/10/09/2024/001\&quot;,\n    \&quot;amount\&quot;: 14000,\n    \&quot;discount\&quot;: 0,\n    \&quot;description\&quot;: \&quot;test saja\&quot;,\n    \&quot;items\&quot; : [\n        {\n            \&quot;productId\&quot;: \&quot;${productID}\&quot;,\n            \&quot;quantity\&quot;: 4,\n            \&quot;cost\&quot;: 3500\n        }\n    ]\n}&quot;,
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
      <webElementGuid>c4739185-2242-4350-8e55-c68cfc2dfbba</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${accessToken}</value>
      <webElementGuid>2b932409-b03e-40b1-a32e-81e06762be57</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/purchases</restUrl>
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
      <id>d11819fe-4ba6-4587-88b1-d69b978c2b87</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessToken</defaultValue>
      <description></description>
      <id>7c57bc16-6709-418e-8992-a5270ab178bd</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.officeID</defaultValue>
      <description></description>
      <id>8871e25b-8e75-4317-bd24-ce2ee34f2806</id>
      <masked>false</masked>
      <name>officeID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productID</defaultValue>
      <description></description>
      <id>3c9ef6b5-7ae3-4a81-b8b8-ef6ce5fb2322</id>
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
