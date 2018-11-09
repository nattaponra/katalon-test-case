<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Test API</name>
   <tag></tag>
   <elementGuidId>8037e709-e0cb-4b0a-98eb-94d67888e3bf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.testing.se.nattaponra.com/?a=${value_a)&amp;b=${value_b)&amp;c=${value_c)</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>1fa2991b-8423-42b1-a9b8-f721fd8c2912</id>
      <masked>false</masked>
      <name>value_a</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>363de995-72ab-44ae-a917-cecb062d369f</id>
      <masked>false</masked>
      <name>value_b</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>424541a5-4209-47ee-b0ea-6328735c5911</id>
      <masked>false</masked>
      <name>value_c</name>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
