import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://staging.segstream.com/account/login/')

WebUI.setText(findTestObject('Object Repository/Page_Login  SegStream/input_Email_email'), 'info@segstream.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login  SegStream/input_Password_password'), 'yr2bZZvQ58ur+DfKXurYpNk5/flLxgCE')

WebUI.click(findTestObject('Object Repository/Page_Login  SegStream/input_Password_btn btn-primary btn-md'))

WebUI.navigateToUrl('https://staging.segstream.com/project/take-offsheet/3340/')

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Add Takeoff or Costs'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _1b3f14'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/a_Spread footings, 3000 PSI concrete, load _baad50'))

WebUI.setText(findTestObject('Object Repository/Page_Worksheet  SegStream/input_Quantity_quantity'), '50')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _adfe60'), 
    '39Y', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _4708c6'), 
    '24065', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _ef802e'), 
    '7310', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _f95f1e'), 
    '1094', true)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Save'))

WebUI.navigateToUrl('https://staging.segstream.com/project/take-offsheet/3340/#')

