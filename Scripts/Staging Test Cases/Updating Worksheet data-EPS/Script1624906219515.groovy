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
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.logging.LogEntries as LogEntries
import org.openqa.selenium.logging.LogEntry as LogEntry
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory

WebUI.openBrowser('', FailureHandling.OPTIONAL)

WebUI.navigateToUrl('https://staging.segstream.com/account/login/')

WebUI.setText(findTestObject('Object Repository/Page_Login  SegStream/input_Email_email'), 'ralphninojasmin@yahoo.com', 
    FailureHandling.OPTIONAL)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Login  SegStream/input_Password_password'), 'p4y+y39Ir5PJb2ispxT0Ew==', 
    FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Page_Login  SegStream/input_Password_btn btn-primary btn-md'), FailureHandling.OPTIONAL)

WebUI.navigateToUrl('https://staging.segstream.com/project/take-offsheet/2750/')

WebUI.doubleClick(findTestObject('Object Repository/Page_Worksheet  SegStream/strong_Site Concrete Paving Slab on grade, _365aab'), 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.doubleClick(findTestObject('Object Repository/Page_Worksheet  SegStream/strong_Site Concrete Paving Slab on grade, _365aab'), 
    FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Page_Worksheet  SegStream/button_Save'))

WebUI.navigateToUrl('https://staging.segstream.com/project/take-offsheet/2750/')

WebUI.verifyMatch(WebUI.getUrl(), 'https://dev.segstream.com/project/take-offsheet/2750/', false)

WebUI.closeBrowser()

