import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://web.testing.se.nattaponra.com/')

WebUI.setText(findTestObject('Object Repository/Test UI Triangle/Page_/input_ A_aSide'), '200')

WebUI.setText(findTestObject('Object Repository/Test UI Triangle/Page_/input_ B_bSide'), '100')

WebUI.setText(findTestObject('Object Repository/Test UI Triangle/Page_/input_ C_cSide'), '100')

WebUI.click(findTestObject('Object Repository/Test UI Triangle/Page_/a_'))

WebUI.click(findTestObject('Object Repository/Test UI Triangle/Page_/div_Not a Triangle'))

WebUI.verifyElementText(findTestObject('Page_/div_Not a Triangle'), 'Not a Triangle')

WebUI.closeBrowser()

