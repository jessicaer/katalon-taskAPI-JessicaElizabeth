<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE Product</name>
   <tag></tag>
   <elementGuidId>ac59025f-c918-4f12-bf87-c60aa220033c</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \t\&quot;category_id\&quot; : \&quot;${categoryID}\&quot;,\n\t\&quot;code\&quot; : \&quot;M01\&quot;,\n    \&quot;name\&quot;: \&quot;Es Teh Manis\&quot;,\n    \&quot;price\&quot;: \&quot;3000\&quot;,\n    \&quot;cost\&quot;: \&quot;2000\&quot;,\n    \&quot;stock\&quot;: \&quot;20\&quot;\n}&quot;,
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
      <webElementGuid>6474c3ac-da75-4a2c-ac41-dd12d25045c7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${accessToken}</value>
      <webElementGuid>d3a7369f-53fb-451b-980b-e2910cdc8f04</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${baseURL}/products/${productID}</restUrl>
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
      <id>03c9c803-1830-45dc-854b-5d851ec21095</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productID</defaultValue>
      <description></description>
      <id>6ff05187-00bd-4263-ada1-22a614eb60e6</id>
      <masked>false</masked>
      <name>productID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessToken</defaultValue>
      <description></description>
      <id>f25f8812-6f0d-4b04-bd17-c723d19db430</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryID</defaultValue>
      <description></description>
      <id>1a76a20d-47bb-4cc4-9590-01855db94554</id>
      <masked>false</masked>
      <name>categoryID</name>
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




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
