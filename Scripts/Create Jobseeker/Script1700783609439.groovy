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

WebUI.callTestCase(findTestCase('Reusables/reusbable_login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementVisible(findTestObject('Object Repository/Page_(edge) job ready/div_ProfileSettingsLogoutAlertsDES AlertsDa_ad5c5d'))

WebUI.click(findTestObject('Object Repository/Page_(edge) job ready/span_Job Seekers'))

WebUI.click(findTestObject('Object Repository/Page_(edge) job ready/a_New Job Seeker'))

WebUI.takeFullPageScreenshotAsCheckpoint('Jobseeker Modal', FailureHandling.STOP_ON_FAILURE)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_(edge) job ready/select_ACEActive but no site managerAdams-B_054971'), 
    '1755', true)

WebUI.setText(findTestObject('Object Repository/Page_(edge) job ready/input_jobseekeress_identifier'), CustomKeywords.'com.RandomNumber.CreateRandomNumber'())

WebUI.setText(findTestObject('Object Repository/Page_(edge) job ready/input_jobseekerfirst_name'), 'Katalon')

WebUI.setText(findTestObject('Object Repository/Page_(edge) job ready/input_jobseekerlast_name'), CustomKeywords.'com.RandomLastName.CreateRandomLastName'())

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_(edge) job ready/select_ABCCCAdvertised VacancyAllied Health_dcf632'), 
    '', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_(edge) job ready/select_ABCCCAdvertised VacancyAllied Health_dcf632'), 
    '40', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_(edge) job ready/select_Allied Health ProviderConsultant Rep_da6cb8'), 
    '13', true)

WebUI.click(findTestObject('Object Repository/Page_(edge) job ready/input_commit'))

WebUI.takeFullPageScreenshotAsCheckpoint('Jobseeker Showpage', FailureHandling.STOP_ON_FAILURE)

WebUI.closeBrowser()

