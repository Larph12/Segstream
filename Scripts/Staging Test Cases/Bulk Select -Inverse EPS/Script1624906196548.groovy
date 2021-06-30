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

WebUI.setText(findTestObject('Object Repository/Bulk Select/Page_Login  SegStream/input_Email_email'), 'ralphninojasmin@yahoo.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Bulk Select/Page_Login  SegStream/input_Password_password'), 'p4y+y39Ir5PJb2ispxT0Ew==')

WebUI.click(findTestObject('Object Repository/Bulk Select/Page_Login  SegStream/input_Password_btn btn-primary btn-md'))

WebUI.navigateToUrl('https://staging.segstream.com/')

WebUI.navigateToUrl('https://staging.segstream.com/project/take-offsheet/3477/')

WebUI.click(findTestObject('Object Repository/Bulk Select Inverse/Page_Worksheet  SegStream/i_Bulk Select_fa fa-chevron-right pull-righ_42c305'))

WebUI.click(findTestObject('Object Repository/Bulk Select Inverse/Page_Worksheet  SegStream/i_Select inverse_fa fa-circle-o'))

WebUI.click(findTestObject('Object Repository/Bulk Select Inverse/Page_Worksheet  SegStream/button_Cancel'))

WebUI.click(findTestObject('Object Repository/Bulk Select Inverse/Page_Worksheet  SegStream/div_ShowHide Children                Toggle_977276'))

WebUI.verifyImagePresent(findTestObject('Object Repository/Bulk Select Inverse/Page_Worksheet  SegStream/div_ShowHide Children                Toggle_977276'))

