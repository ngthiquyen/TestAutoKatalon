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

WebUI.navigateToUrl('https://thegioiskinfood.com/')

WebUI.getText(findTestObject('Object Repository/GioHang/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/p_Mu 1-17 Son Tint Nc Romand Siu L, Lu Tri _c0cffc'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/a_Mu 1-17 Son Tint Nc Romand Siu L, Lu Tri _558ed2_1'))

WebUI.verifyElementPresent(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/h1_Mu 1-17 Son Tint Nc Romand Siu L, Lu Tri_787bc3'), 
    0)

WebUI.click(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/span_03  GCH'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/button_'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/button_-'))

WebUI.click(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/button_Thm vo gi'))

WebUI.verifyElementPresent(findTestObject('Object Repository/GioHang/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/span_Sn phm  c thm vo gi hng_1'), 
    0)

TestObject cart = findTestObject('Object Repository/GioHang/cart')

WebUI.scrollToElement(cart, 5)

WebUI.waitForElementClickable(cart, 10)

WebUI.click(cart)

WebUI.verifyElementPresent(findTestObject('Object Repository/GioHang/Page_Gi hng ca bn - TH GII SKINFOOD/h3_Mu 1-17 Son Tint Nc Romand Siu L, Lu Tri_a888eb'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/GioHang/Page_Gi hng ca bn - TH GII SKINFOOD/span_179.000_1'), 
    0)

WebUI.closeBrowser()

