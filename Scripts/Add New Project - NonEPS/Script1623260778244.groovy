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

WebUI.setText(findTestObject('Object Repository/Page_Login  SegStream/input_Email_email'), 'info@segstream.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login  SegStream/input_Password_password'), 'yr2bZZvQ58ur+DfKXurYpNk5/flLxgCE')

WebUI.click(findTestObject('Object Repository/Page_Login  SegStream/input_Password_btn btn-primary btn-md'))

WebUI.click(findTestObject('Object Repository/Page_Dashboard  SegStream/button_Add Project'))

WebUI.setText(findTestObject('Object Repository/Page_Dashboard  SegStream/input_Project Name_title'), 'April Test Project')

WebUI.click(findTestObject('Object Repository/Page_Dashboard  SegStream/button_Save'))

WebUI.setText(findTestObject('Object Repository/Page_Create Project Step1  SegStream/textarea_(Note Hit enter to add multiple en_0897f8'), 
    'Test_1')

WebUI.click(findTestObject('Object Repository/Page_Create Project Step1  SegStream/input_(Note Hit enter to add multiple entit_2f1426'))

WebUI.setText(findTestObject('Object Repository/Page_Create Project step2  SegStream/input_Name_name'), 'April 21')

WebUI.setText(findTestObject('Object Repository/Page_Create Project step2  SegStream/input_Zip Code_zipcode'), '50001')

WebUI.click(findTestObject('Object Repository/Page_Create Project step2  SegStream/button_Add Property'))

WebUI.closeBrowser()

