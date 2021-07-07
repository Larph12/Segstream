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

WebUI.navigateToUrl('https://dev.segstream.com/account/login/')

WebUI.setText(findTestObject('Object Repository/Page_Login  SegStream/input_Email_email'), 'ralphninojasmin@yahoo.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login  SegStream/input_Password_password'), 'p4y+y39Ir5PJb2ispxT0Ew==')

WebUI.click(findTestObject('Object Repository/Page_Login  SegStream/input_Password_btn btn-primary btn-md'))

WebUI.navigateToUrl('https://dev.segstream.com/project/take-offsheet/2749/')

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/a_Bulk Modify'))

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/a_Modify Recovery Period of Selected'))

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_'))

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/a_Bulk Select'))

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/a_Select By Recovery Period'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Select                              _8ca40b'), 
    '5Y', true)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Select Items'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_Clone                               _698ed6'), 
    'update_asset_year', true)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Select'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Worksheet  SegStream/select_5 Years                             _9f2aac'), 
    '15Y', true)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Update'))

WebUI.navigateToUrl('https://dev.segstream.com/project/take-offsheet/2749/?page=input_order')

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/div_Filter     User Working Status         _cc749d'))

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_Worksheet  SegStream/div_Filter     User Working Status         _cc749d'))

