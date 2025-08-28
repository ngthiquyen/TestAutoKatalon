<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_MERZYNew Dew Son Tint Bng Merzy Dng Thc_0eacc9</name>
   <tag></tag>
   <elementGuidId>7f136dd4-c4d3-4b46-9625-b73e05dc6fb1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.page-product-info-right</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='page-product']/section/div/div/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>bcef9248-2bd5-4cc8-a16b-290252488800</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page-product-info-right</value>
      <webElementGuid>0f5f5895-e741-4919-8187-33db4ea5a57c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
					
						

						
					

					
						
              
							
								MERZY
								
									
								
							
						
						
							[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
						
            
					 
          
					
						
							
							0 đánh giá
							Chưa có hỏi đáp
  							50 Đã bán
						
						
							
								
									
								
							
						
					
          
					
						
  							FREESHIP HCM HN

              


              
  							NEW
              

              

						
						
							Thông tin Outlet
						
					
          
					 
    						 
    							KHÔNG ÁP DỤNG ƯU ĐÃI KHÁC.
    						 
    						 
    							GIÁ CHỈ 157K/SON KHI NHẬP MÃ TGSFWD10PT
    						 
					
          
					
						
							299,000₫
							172,000₫
						

.ae-popup-price-com{position:absolute;border:1px solid #ccc;border-radius:10px;background:#fff;max-width:380px;padding:10px 10px 7px 10px;box-shadow:0 2px 5px rgba(0,0,0,.2);visibility:hidden;z-index:-1;top:100%;opacity:0;transition:all .2s cubic-bezier(0.4, 0, 0.2, 1);left:8px;width:calc(100% - 16px)}.ae-popup-price-com::before{width:7px;height:7px;content:&quot;&quot;;background:#fff;border-left:1px solid #ccc;border-top:1px solid #ccc;transform:rotate(45deg);position:absolute;left:26px;top:-4px}.ae-popup-price-com .ae-popup-content{font-size:95%}.ae-popup-price-com .ae-popup-content .ae-divide{border-bottom:1px dashed #bdbdbd}.ae-popup-price-com .ae-popup-content .ae-last-price{font-size:120%}.ae-popup-price-com .ae-popup-content p{margin:8px 0}.ae-popup-price-com .ae-popup-content h4{font-size:130%;margin:8px 0 10px 0}.ae-fr{float:right}.ae-bold{font-weight:700}.ae-text-highlight{color:#820813}.ae-text-secondary{color:#868686}.ae-text-orange{color:#a5751e}.d-none{display:none}.page-product-info-price{position:relative;transition:all .2s ease-in-out}#ae-tag-img{display:none}#ae-tag-price{max-width:28px;display:block;position:relative;margin-top:-2px;margin-right:3px;float:left}




  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  



  var aeVariantSelected = &quot;&quot;;
  if ((typeof Aecom) === 'undefined') {
    Aecom = {};
  }

  Aecom.changeOption = function () {
    $(&quot;.page-product-info-pricewrap #ae-tag-price&quot;).remove();
    const value = $(&quot;#product-select option:selected&quot;).val()
    $(&quot;#product-select option&quot;).each(function() {
      if ($(this).val() == value) {
        // console.log($(this).val(), $(this).text())
        $('.page-product-info-price[class*=&quot;variant-popup-&quot;]').attr('class', function(i, val){
            return val.replace(/(^|\s)variant-popup\S+/g, '');
        });
        const clsActiveVariant = &quot;variant-popup-&quot; + $(this).val()
        const selectorSelected = &quot;.ae-popup-price-com.&quot; + clsActiveVariant
        console.log(selectorSelected)
        $(&quot;.page-product-info-price&quot;).addClass(clsActiveVariant)
        
        if ($(selectorSelected).length > 0) {
          $(&quot;.page-product-info-price .page-product-info-pricewrap .page-product-info-newprice>span&quot;).before(&quot;&lt;span id='ae-tag-price'>&lt;/span>&quot;);
          $(&quot;#ae-tag-price&quot;).html($(&quot;#ae-tag-img&quot;).html())
        }
      }
    })
  }
  Number.prototype.formatMoney = function (c, d, t) {
  	var n = this,
  		c = isNaN((c = Math.abs(c))) ? 2 : c,
  		d = d == undefined ? &quot;,&quot; : d,
  		t = t == undefined ? &quot;.&quot; : t,
  		s = n &lt; 0 ? &quot;-&quot; : &quot;&quot;,
  		i = parseInt((n = Math.abs(+n || 0).toFixed(c))) + &quot;&quot;,
  		j = (j = i.length) > 3 ? j % 3 : 0;
  	return (
  		s +
  		(j ? i.substr(0, j) + t : &quot;&quot;) +
  		i.substr(j).replace(/(\d{3})(?=\d)/g, &quot;$1&quot; + t) +
  		(c
  			? d +
  			  Math.abs(n - i)
  					.toFixed(c)
  					.slice(2)
  			: &quot;&quot;)
  	);
  };
  Aecom.formatNumber = function(value) {
    if (!value) return '0 đ'
    const number = Number(value).toFixed(0)
    const trimNumber = Number(number) + ''
    return trimNumber.replace(/\B(?=(\d{3})+(?!\d))/g, ',') + ' đ'
  }
  
  Aecom.showPopupPrice = function() {
    if (aeVariantSelected) $(&quot;.page-product-info-price&quot;).removeClass(&quot;variant-&quot; + aeVariantSelected)
    
    aeVariantSelected = $(&quot;#product-select&quot;).val();
    // console.log(aeVariantSelected)
    $(&quot;.page-product-info-price&quot;).addClass(&quot;variant-&quot; + aeVariantSelected)
  }
  document.addEventListener('DOMContentLoaded', () => {
    $(&quot;.ae-price-compare&quot;).each(function() {
      const cover = $(this).closest(&quot;.ae-popup-content&quot;)
      const priceCompare = parseFloat($(cover).find(&quot;.ae-price-compare&quot;).data(&quot;price&quot;)) / 100
      let priceRestore = parseFloat($(cover).find(&quot;.ae-restore-price&quot;).data(&quot;price&quot;)) / 100
      const priceSale = parseFloat($(cover).find(&quot;.ae-price-sale&quot;).data(&quot;price&quot;)) / 100
      priceRestore = priceSale > priceRestore ? priceCompare : priceRestore;
      const discount1 = priceCompare - priceRestore
      const discount2 = priceRestore - priceSale
      
      $(cover).find(&quot;.ae-price-compare&quot;).text(Aecom.formatNumber(priceCompare))
      $(cover).find(&quot;.ae-restore-price&quot;).text(Aecom.formatNumber(priceRestore))
      $(cover).find(&quot;.ae-price-sale&quot;).text(Aecom.formatNumber(priceSale))
      $(cover).find(&quot;.ae-price-discount-1&quot;).text(Aecom.formatNumber(discount1))
      $(cover).find(&quot;.ae-price-discount-2&quot;).text(Aecom.formatNumber(discount2))
      
    });

     setTimeout(function() {
       Aecom.changeOption()
     }, 4000);
  });

						
							-42%
							GIFT
						
            
						
							CÓ TÍCH ĐIỂM
						
					
          


					
						Voucher Hot
						
					

					
            

					
          
					
						WD14 WITH ZESTWD20 MISTY WOODWD21 BURNT MAPLEWD22 HAZEL CHILIWD23 ANTIQUE FLAMEWD26 HUSHED APPEALWD27 FIG DUSTYWD28 WHISPER ODDYWD29 FRESH BLOOMWD30 BLISS DAWN









  							WD14 WITH ZEST - 172,000₫









  							WD20 MISTY WOOD - 172,000₫









  							WD21 BURNT MAPLE - 172,000₫









  							WD22 HAZEL CHILI - 172,000₫









  							WD23 ANTIQUE FLAME - 172,000₫









  							WD26 HUSHED APPEAL - 172,000₫









  							WD27 FIG DUSTY - 172,000₫









  							WD28 WHISPER ODDY - 172,000₫









  							WD29 FRESH BLOOM - 172,000₫









  							WD30 BLISS DAWN - 172,000₫
						
					
          
					
						
							
								
							
							
								
									
										
									
								
								
									[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
								
								
									299,000₫
									172,000₫
								
								
									
										
											Số lượng: 17
										
										
											Thông tin Outlet
										
									

								
								
						
						
							




	
		Tiêu đề
	
	
		
		 
		
		
		 
		
		
		

			
			
				
				WD14 WITH ZEST 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD20 MISTY WOOD 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD21 BURNT MAPLE 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD22 HAZEL CHILI 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD23 ANTIQUE FLAME 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD26 HUSHED APPEAL 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD27 FIG DUSTY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD28 WHISPER ODDY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD29 FRESH BLOOM 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD30 BLISS DAWN 
			
			
		
		
	


							
								Vui lòng chọn biến thể
							
						
						
							Số lượng
							
								-
								
								+
							
						
					 
          
					
						WD14 WITH ZEST
						



    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							
						
					
          

					

					
						
							Nhắc nhở tôi
						
            
  						
  							Mua Online
  						

						
							Thêm vào giỏ
						


						
							
								
									
								
								10/11 chi nhánh còn hàng
							
							
								
									
										Bạn vui lòng chọn biến thể
									
									
										
											
												ONLINE
											
											
												- Còn hàng (giao HOẢ TỐC 2H HCM, 24h toàn quốc)
												- Sắp về hàng(giao HOẢ TỐC 2H HCM, 24h toàn quốc)
											
										
										
										
											
												OFFLINE - CN Hồ Chí Minh
											
											
												
												
													
														
														Quận 3
													
													
														
														- Còn hàng  365 Lê Văn Sỹ, Phường 12
														
													 
												
												
												
													
														
														Quận 5
													
													
														
														- Còn hàng  159 Nguyễn Văn Cừ, Phường 2
														
													 
												
												
												
													
														
														Quận Tân Bình
													
													
														
														- Còn hàng  100 Hoàng Hoa Thám, Phường 12
														
													 
												
												
												
													
														
														Quận Bình Thạnh
													
													
														
														- Còn hàng  94 D5, Phường 25
														
													 
												
												
												
													
														
														Quận Gò Vấp
													
													
														
														- Còn hàng  15 Phan Văn Trị, phường 7
														
													 
												
												
												
													
														
														TP. Thủ Đức
													
													
														
														- Còn hàng  366 Võ Văn Ngân, phường Bình Thọ
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN MỸ THO - TIỀN GIANG
											
											
												
												
													
														
														TP Mỹ Tho - Tiền Giang
													
													
														
														- Sắp về hàng  Tầng L1, TTTM Vincom Plaza, 1A Hùng Vương, Phường 1
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN ĐỒNG NAI - BIÊN HÒA
											
											
												
												
													
														
														TP Biên Hòa - Đồng Nai
													
													
														
														- Còn hàng  278 Võ Thị Sáu, phường Thống Nhất
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HÀ NỘI
											
											
												
												
													
														
														NAM TỪ LIÊM
													
													
														
														- Còn hàng  240 Nguyễn Trãi, phường Trung Văn (cách Đại học Hà Nội 50m)
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HUẾ
											
											
												
												
													
														
														TP.Huế - Thừa Thiên Huế
													
													
														
														- Còn hàng  Tầng 1, TTTM  AEON MALL, 08 Võ Nguyên Giáp, phường An Đông
														
													 
												
												
											
										
										
									
								
							
						
					

          

          
            
              
                                QUÀ TẶNG KHUYẾN MÃI                     
                        
                          
                            
                              
                            
                          
                          
                            MUA 1 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 1 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Túi Phao Make Up Merzy New Dew Bag (màu ngẫu nhiên) - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Lịch Để Bàn Merzy - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                                         
          
          
							
  
    
      Các sản phẩm được mua kèm						
      Chọn 1 trong các loại sản phẩm sau
    							
    
      									
        MUA KÈMMua 1 [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g để nhận ngay ưu đãi đi kèm (Tối đa 3)Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Hồng15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Dương15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Lá15,000₫Mua ngay
      
    
  


	
		Chương trình khuyến mãi
		

		
	


	
		Khuyến mãi
	 
	 
											
			
		
	

          
  					
  						var price=172000;
  						
  						   Mua trả sau 0 ₫ với                                                                 .cls-1 { fill:#212121; } .cls-2 { fill:#fff; }               .promo-cls-1 { fill:#fff; }            Giảm đến 50K khi thanh toán qua Fundiin. Xem thêm     ×     
  						
              
              
              
              
  							
  								Miễn phí thanh toán
  								
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
  									
  										Chi tiết 
  											
  												
  											
  										
  									
  								
  							
  							
  								
  									Ẩn bớt
  										
  											
  										
  									
  								
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví VNPAY - Miễn phí thanh toán
      										Nhập mã: VNPAYSKIN25  Giảm 10K cho đơn hàng từ 399K  Áp dụng 14/03/2025 - 30/06/2025
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán qua Fundiin - Mua trước trả sau
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ShopeePay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ZaloPay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví Momo - Miễn phí thanh toán
      										
      									
      								
                    
                  
  							
    					
  					

					
          
				</value>
      <webElementGuid>6e4441cb-9e2a-4a42-b19f-3eb5e57ffd78</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;page-product&quot;)/section[@class=&quot;page-product-info&quot;]/div[@class=&quot;container&quot;]/div[@class=&quot;page-product-info-wrap&quot;]/div[@class=&quot;page-product-info-right&quot;]</value>
      <webElementGuid>b6ea26af-2a49-4136-bfb5-c49ef437f7bb</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='page-product']/section/div/div/div[2]</value>
      <webElementGuid>64b29fff-f779-4b2d-82de-0c21dc50d070</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='/16'])[1]/following::div[3]</value>
      <webElementGuid>5e50791e-abb5-49ec-9568-5c29181a00c3</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='SẮP VỀ HÀNG'])[1]/following::div[25]</value>
      <webElementGuid>e7414837-5eef-4703-823d-49b2a69f9209</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/section/div/div/div[2]</value>
      <webElementGuid>fa2bc634-7dac-47da-b076-9bf14c136f45</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
					
						

						
					

					
						
              
							
								MERZY
								
									
								
							
						
						
							[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
						
            
					 
          
					
						
							
							0 đánh giá
							Chưa có hỏi đáp
  							50 Đã bán
						
						
							
								
									
								
							
						
					
          
					
						
  							FREESHIP HCM HN

              


              
  							NEW
              

              

						
						
							Thông tin Outlet
						
					
          
					 
    						 
    							KHÔNG ÁP DỤNG ƯU ĐÃI KHÁC.
    						 
    						 
    							GIÁ CHỈ 157K/SON KHI NHẬP MÃ TGSFWD10PT
    						 
					
          
					
						
							299,000₫
							172,000₫
						

.ae-popup-price-com{position:absolute;border:1px solid #ccc;border-radius:10px;background:#fff;max-width:380px;padding:10px 10px 7px 10px;box-shadow:0 2px 5px rgba(0,0,0,.2);visibility:hidden;z-index:-1;top:100%;opacity:0;transition:all .2s cubic-bezier(0.4, 0, 0.2, 1);left:8px;width:calc(100% - 16px)}.ae-popup-price-com::before{width:7px;height:7px;content:&quot;&quot;;background:#fff;border-left:1px solid #ccc;border-top:1px solid #ccc;transform:rotate(45deg);position:absolute;left:26px;top:-4px}.ae-popup-price-com .ae-popup-content{font-size:95%}.ae-popup-price-com .ae-popup-content .ae-divide{border-bottom:1px dashed #bdbdbd}.ae-popup-price-com .ae-popup-content .ae-last-price{font-size:120%}.ae-popup-price-com .ae-popup-content p{margin:8px 0}.ae-popup-price-com .ae-popup-content h4{font-size:130%;margin:8px 0 10px 0}.ae-fr{float:right}.ae-bold{font-weight:700}.ae-text-highlight{color:#820813}.ae-text-secondary{color:#868686}.ae-text-orange{color:#a5751e}.d-none{display:none}.page-product-info-price{position:relative;transition:all .2s ease-in-out}#ae-tag-img{display:none}#ae-tag-price{max-width:28px;display:block;position:relative;margin-top:-2px;margin-right:3px;float:left}




  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  



  var aeVariantSelected = &quot;&quot;;
  if ((typeof Aecom) === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
    Aecom = {};
  }

  Aecom.changeOption = function () {
    $(&quot;.page-product-info-pricewrap #ae-tag-price&quot;).remove();
    const value = $(&quot;#product-select option:selected&quot;).val()
    $(&quot;#product-select option&quot;).each(function() {
      if ($(this).val() == value) {
        // console.log($(this).val(), $(this).text())
        $(&quot; , &quot;'&quot; , &quot;.page-product-info-price[class*=&quot;variant-popup-&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, function(i, val){
            return val.replace(/(^|\s)variant-popup\S+/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        });
        const clsActiveVariant = &quot;variant-popup-&quot; + $(this).val()
        const selectorSelected = &quot;.ae-popup-price-com.&quot; + clsActiveVariant
        console.log(selectorSelected)
        $(&quot;.page-product-info-price&quot;).addClass(clsActiveVariant)
        
        if ($(selectorSelected).length > 0) {
          $(&quot;.page-product-info-price .page-product-info-pricewrap .page-product-info-newprice>span&quot;).before(&quot;&lt;span id=&quot; , &quot;'&quot; , &quot;ae-tag-price&quot; , &quot;'&quot; , &quot;>&lt;/span>&quot;);
          $(&quot;#ae-tag-price&quot;).html($(&quot;#ae-tag-img&quot;).html())
        }
      }
    })
  }
  Number.prototype.formatMoney = function (c, d, t) {
  	var n = this,
  		c = isNaN((c = Math.abs(c))) ? 2 : c,
  		d = d == undefined ? &quot;,&quot; : d,
  		t = t == undefined ? &quot;.&quot; : t,
  		s = n &lt; 0 ? &quot;-&quot; : &quot;&quot;,
  		i = parseInt((n = Math.abs(+n || 0).toFixed(c))) + &quot;&quot;,
  		j = (j = i.length) > 3 ? j % 3 : 0;
  	return (
  		s +
  		(j ? i.substr(0, j) + t : &quot;&quot;) +
  		i.substr(j).replace(/(\d{3})(?=\d)/g, &quot;$1&quot; + t) +
  		(c
  			? d +
  			  Math.abs(n - i)
  					.toFixed(c)
  					.slice(2)
  			: &quot;&quot;)
  	);
  };
  Aecom.formatNumber = function(value) {
    if (!value) return &quot; , &quot;'&quot; , &quot;0 đ&quot; , &quot;'&quot; , &quot;
    const number = Number(value).toFixed(0)
    const trimNumber = Number(number) + &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    return trimNumber.replace(/\B(?=(\d{3})+(?!\d))/g, &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; đ&quot; , &quot;'&quot; , &quot;
  }
  
  Aecom.showPopupPrice = function() {
    if (aeVariantSelected) $(&quot;.page-product-info-price&quot;).removeClass(&quot;variant-&quot; + aeVariantSelected)
    
    aeVariantSelected = $(&quot;#product-select&quot;).val();
    // console.log(aeVariantSelected)
    $(&quot;.page-product-info-price&quot;).addClass(&quot;variant-&quot; + aeVariantSelected)
  }
  document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, () => {
    $(&quot;.ae-price-compare&quot;).each(function() {
      const cover = $(this).closest(&quot;.ae-popup-content&quot;)
      const priceCompare = parseFloat($(cover).find(&quot;.ae-price-compare&quot;).data(&quot;price&quot;)) / 100
      let priceRestore = parseFloat($(cover).find(&quot;.ae-restore-price&quot;).data(&quot;price&quot;)) / 100
      const priceSale = parseFloat($(cover).find(&quot;.ae-price-sale&quot;).data(&quot;price&quot;)) / 100
      priceRestore = priceSale > priceRestore ? priceCompare : priceRestore;
      const discount1 = priceCompare - priceRestore
      const discount2 = priceRestore - priceSale
      
      $(cover).find(&quot;.ae-price-compare&quot;).text(Aecom.formatNumber(priceCompare))
      $(cover).find(&quot;.ae-restore-price&quot;).text(Aecom.formatNumber(priceRestore))
      $(cover).find(&quot;.ae-price-sale&quot;).text(Aecom.formatNumber(priceSale))
      $(cover).find(&quot;.ae-price-discount-1&quot;).text(Aecom.formatNumber(discount1))
      $(cover).find(&quot;.ae-price-discount-2&quot;).text(Aecom.formatNumber(discount2))
      
    });

     setTimeout(function() {
       Aecom.changeOption()
     }, 4000);
  });

						
							-42%
							GIFT
						
            
						
							CÓ TÍCH ĐIỂM
						
					
          


					
						Voucher Hot
						
					

					
            

					
          
					
						WD14 WITH ZESTWD20 MISTY WOODWD21 BURNT MAPLEWD22 HAZEL CHILIWD23 ANTIQUE FLAMEWD26 HUSHED APPEALWD27 FIG DUSTYWD28 WHISPER ODDYWD29 FRESH BLOOMWD30 BLISS DAWN









  							WD14 WITH ZEST - 172,000₫









  							WD20 MISTY WOOD - 172,000₫









  							WD21 BURNT MAPLE - 172,000₫









  							WD22 HAZEL CHILI - 172,000₫









  							WD23 ANTIQUE FLAME - 172,000₫









  							WD26 HUSHED APPEAL - 172,000₫









  							WD27 FIG DUSTY - 172,000₫









  							WD28 WHISPER ODDY - 172,000₫









  							WD29 FRESH BLOOM - 172,000₫









  							WD30 BLISS DAWN - 172,000₫
						
					
          
					
						
							
								
							
							
								
									
										
									
								
								
									[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
								
								
									299,000₫
									172,000₫
								
								
									
										
											Số lượng: 17
										
										
											Thông tin Outlet
										
									

								
								
						
						
							




	
		Tiêu đề
	
	
		
		 
		
		
		 
		
		
		

			
			
				
				WD14 WITH ZEST 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD20 MISTY WOOD 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD21 BURNT MAPLE 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD22 HAZEL CHILI 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD23 ANTIQUE FLAME 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD26 HUSHED APPEAL 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD27 FIG DUSTY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD28 WHISPER ODDY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD29 FRESH BLOOM 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD30 BLISS DAWN 
			
			
		
		
	


							
								Vui lòng chọn biến thể
							
						
						
							Số lượng
							
								-
								
								+
							
						
					 
          
					
						WD14 WITH ZEST
						



    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							
						
					
          

					

					
						
							Nhắc nhở tôi
						
            
  						
  							Mua Online
  						

						
							Thêm vào giỏ
						


						
							
								
									
								
								10/11 chi nhánh còn hàng
							
							
								
									
										Bạn vui lòng chọn biến thể
									
									
										
											
												ONLINE
											
											
												- Còn hàng (giao HOẢ TỐC 2H HCM, 24h toàn quốc)
												- Sắp về hàng(giao HOẢ TỐC 2H HCM, 24h toàn quốc)
											
										
										
										
											
												OFFLINE - CN Hồ Chí Minh
											
											
												
												
													
														
														Quận 3
													
													
														
														- Còn hàng  365 Lê Văn Sỹ, Phường 12
														
													 
												
												
												
													
														
														Quận 5
													
													
														
														- Còn hàng  159 Nguyễn Văn Cừ, Phường 2
														
													 
												
												
												
													
														
														Quận Tân Bình
													
													
														
														- Còn hàng  100 Hoàng Hoa Thám, Phường 12
														
													 
												
												
												
													
														
														Quận Bình Thạnh
													
													
														
														- Còn hàng  94 D5, Phường 25
														
													 
												
												
												
													
														
														Quận Gò Vấp
													
													
														
														- Còn hàng  15 Phan Văn Trị, phường 7
														
													 
												
												
												
													
														
														TP. Thủ Đức
													
													
														
														- Còn hàng  366 Võ Văn Ngân, phường Bình Thọ
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN MỸ THO - TIỀN GIANG
											
											
												
												
													
														
														TP Mỹ Tho - Tiền Giang
													
													
														
														- Sắp về hàng  Tầng L1, TTTM Vincom Plaza, 1A Hùng Vương, Phường 1
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN ĐỒNG NAI - BIÊN HÒA
											
											
												
												
													
														
														TP Biên Hòa - Đồng Nai
													
													
														
														- Còn hàng  278 Võ Thị Sáu, phường Thống Nhất
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HÀ NỘI
											
											
												
												
													
														
														NAM TỪ LIÊM
													
													
														
														- Còn hàng  240 Nguyễn Trãi, phường Trung Văn (cách Đại học Hà Nội 50m)
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HUẾ
											
											
												
												
													
														
														TP.Huế - Thừa Thiên Huế
													
													
														
														- Còn hàng  Tầng 1, TTTM  AEON MALL, 08 Võ Nguyên Giáp, phường An Đông
														
													 
												
												
											
										
										
									
								
							
						
					

          

          
            
              
                                QUÀ TẶNG KHUYẾN MÃI                     
                        
                          
                            
                              
                            
                          
                          
                            MUA 1 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 1 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Túi Phao Make Up Merzy New Dew Bag (màu ngẫu nhiên) - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Lịch Để Bàn Merzy - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                                         
          
          
							
  
    
      Các sản phẩm được mua kèm						
      Chọn 1 trong các loại sản phẩm sau
    							
    
      									
        MUA KÈMMua 1 [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g để nhận ngay ưu đãi đi kèm (Tối đa 3)Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Hồng15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Dương15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Lá15,000₫Mua ngay
      
    
  


	
		Chương trình khuyến mãi
		

		
	


	
		Khuyến mãi
	 
	 
											
			
		
	

          
  					
  						var price=172000;
  						
  						   Mua trả sau 0 ₫ với                                                                 .cls-1 { fill:#212121; } .cls-2 { fill:#fff; }               .promo-cls-1 { fill:#fff; }            Giảm đến 50K khi thanh toán qua Fundiin. Xem thêm     ×     
  						
              
              
              
              
  							
  								Miễn phí thanh toán
  								
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
  									
  										Chi tiết 
  											
  												
  											
  										
  									
  								
  							
  							
  								
  									Ẩn bớt
  										
  											
  										
  									
  								
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví VNPAY - Miễn phí thanh toán
      										Nhập mã: VNPAYSKIN25  Giảm 10K cho đơn hàng từ 399K  Áp dụng 14/03/2025 - 30/06/2025
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán qua Fundiin - Mua trước trả sau
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ShopeePay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ZaloPay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví Momo - Miễn phí thanh toán
      										
      									
      								
                    
                  
  							
    					
  					

					
          
				&quot;) or . = concat(&quot;
					
						

						
					

					
						
              
							
								MERZY
								
									
								
							
						
						
							[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
						
            
					 
          
					
						
							
							0 đánh giá
							Chưa có hỏi đáp
  							50 Đã bán
						
						
							
								
									
								
							
						
					
          
					
						
  							FREESHIP HCM HN

              


              
  							NEW
              

              

						
						
							Thông tin Outlet
						
					
          
					 
    						 
    							KHÔNG ÁP DỤNG ƯU ĐÃI KHÁC.
    						 
    						 
    							GIÁ CHỈ 157K/SON KHI NHẬP MÃ TGSFWD10PT
    						 
					
          
					
						
							299,000₫
							172,000₫
						

.ae-popup-price-com{position:absolute;border:1px solid #ccc;border-radius:10px;background:#fff;max-width:380px;padding:10px 10px 7px 10px;box-shadow:0 2px 5px rgba(0,0,0,.2);visibility:hidden;z-index:-1;top:100%;opacity:0;transition:all .2s cubic-bezier(0.4, 0, 0.2, 1);left:8px;width:calc(100% - 16px)}.ae-popup-price-com::before{width:7px;height:7px;content:&quot;&quot;;background:#fff;border-left:1px solid #ccc;border-top:1px solid #ccc;transform:rotate(45deg);position:absolute;left:26px;top:-4px}.ae-popup-price-com .ae-popup-content{font-size:95%}.ae-popup-price-com .ae-popup-content .ae-divide{border-bottom:1px dashed #bdbdbd}.ae-popup-price-com .ae-popup-content .ae-last-price{font-size:120%}.ae-popup-price-com .ae-popup-content p{margin:8px 0}.ae-popup-price-com .ae-popup-content h4{font-size:130%;margin:8px 0 10px 0}.ae-fr{float:right}.ae-bold{font-weight:700}.ae-text-highlight{color:#820813}.ae-text-secondary{color:#868686}.ae-text-orange{color:#a5751e}.d-none{display:none}.page-product-info-price{position:relative;transition:all .2s ease-in-out}#ae-tag-img{display:none}#ae-tag-price{max-width:28px;display:block;position:relative;margin-top:-2px;margin-right:3px;float:left}




  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  
    
    
    
      
    
  



  var aeVariantSelected = &quot;&quot;;
  if ((typeof Aecom) === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
    Aecom = {};
  }

  Aecom.changeOption = function () {
    $(&quot;.page-product-info-pricewrap #ae-tag-price&quot;).remove();
    const value = $(&quot;#product-select option:selected&quot;).val()
    $(&quot;#product-select option&quot;).each(function() {
      if ($(this).val() == value) {
        // console.log($(this).val(), $(this).text())
        $(&quot; , &quot;'&quot; , &quot;.page-product-info-price[class*=&quot;variant-popup-&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, function(i, val){
            return val.replace(/(^|\s)variant-popup\S+/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        });
        const clsActiveVariant = &quot;variant-popup-&quot; + $(this).val()
        const selectorSelected = &quot;.ae-popup-price-com.&quot; + clsActiveVariant
        console.log(selectorSelected)
        $(&quot;.page-product-info-price&quot;).addClass(clsActiveVariant)
        
        if ($(selectorSelected).length > 0) {
          $(&quot;.page-product-info-price .page-product-info-pricewrap .page-product-info-newprice>span&quot;).before(&quot;&lt;span id=&quot; , &quot;'&quot; , &quot;ae-tag-price&quot; , &quot;'&quot; , &quot;>&lt;/span>&quot;);
          $(&quot;#ae-tag-price&quot;).html($(&quot;#ae-tag-img&quot;).html())
        }
      }
    })
  }
  Number.prototype.formatMoney = function (c, d, t) {
  	var n = this,
  		c = isNaN((c = Math.abs(c))) ? 2 : c,
  		d = d == undefined ? &quot;,&quot; : d,
  		t = t == undefined ? &quot;.&quot; : t,
  		s = n &lt; 0 ? &quot;-&quot; : &quot;&quot;,
  		i = parseInt((n = Math.abs(+n || 0).toFixed(c))) + &quot;&quot;,
  		j = (j = i.length) > 3 ? j % 3 : 0;
  	return (
  		s +
  		(j ? i.substr(0, j) + t : &quot;&quot;) +
  		i.substr(j).replace(/(\d{3})(?=\d)/g, &quot;$1&quot; + t) +
  		(c
  			? d +
  			  Math.abs(n - i)
  					.toFixed(c)
  					.slice(2)
  			: &quot;&quot;)
  	);
  };
  Aecom.formatNumber = function(value) {
    if (!value) return &quot; , &quot;'&quot; , &quot;0 đ&quot; , &quot;'&quot; , &quot;
    const number = Number(value).toFixed(0)
    const trimNumber = Number(number) + &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    return trimNumber.replace(/\B(?=(\d{3})+(?!\d))/g, &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; đ&quot; , &quot;'&quot; , &quot;
  }
  
  Aecom.showPopupPrice = function() {
    if (aeVariantSelected) $(&quot;.page-product-info-price&quot;).removeClass(&quot;variant-&quot; + aeVariantSelected)
    
    aeVariantSelected = $(&quot;#product-select&quot;).val();
    // console.log(aeVariantSelected)
    $(&quot;.page-product-info-price&quot;).addClass(&quot;variant-&quot; + aeVariantSelected)
  }
  document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, () => {
    $(&quot;.ae-price-compare&quot;).each(function() {
      const cover = $(this).closest(&quot;.ae-popup-content&quot;)
      const priceCompare = parseFloat($(cover).find(&quot;.ae-price-compare&quot;).data(&quot;price&quot;)) / 100
      let priceRestore = parseFloat($(cover).find(&quot;.ae-restore-price&quot;).data(&quot;price&quot;)) / 100
      const priceSale = parseFloat($(cover).find(&quot;.ae-price-sale&quot;).data(&quot;price&quot;)) / 100
      priceRestore = priceSale > priceRestore ? priceCompare : priceRestore;
      const discount1 = priceCompare - priceRestore
      const discount2 = priceRestore - priceSale
      
      $(cover).find(&quot;.ae-price-compare&quot;).text(Aecom.formatNumber(priceCompare))
      $(cover).find(&quot;.ae-restore-price&quot;).text(Aecom.formatNumber(priceRestore))
      $(cover).find(&quot;.ae-price-sale&quot;).text(Aecom.formatNumber(priceSale))
      $(cover).find(&quot;.ae-price-discount-1&quot;).text(Aecom.formatNumber(discount1))
      $(cover).find(&quot;.ae-price-discount-2&quot;).text(Aecom.formatNumber(discount2))
      
    });

     setTimeout(function() {
       Aecom.changeOption()
     }, 4000);
  });

						
							-42%
							GIFT
						
            
						
							CÓ TÍCH ĐIỂM
						
					
          


					
						Voucher Hot
						
					

					
            

					
          
					
						WD14 WITH ZESTWD20 MISTY WOODWD21 BURNT MAPLEWD22 HAZEL CHILIWD23 ANTIQUE FLAMEWD26 HUSHED APPEALWD27 FIG DUSTYWD28 WHISPER ODDYWD29 FRESH BLOOMWD30 BLISS DAWN









  							WD14 WITH ZEST - 172,000₫









  							WD20 MISTY WOOD - 172,000₫









  							WD21 BURNT MAPLE - 172,000₫









  							WD22 HAZEL CHILI - 172,000₫









  							WD23 ANTIQUE FLAME - 172,000₫









  							WD26 HUSHED APPEAL - 172,000₫









  							WD27 FIG DUSTY - 172,000₫









  							WD28 WHISPER ODDY - 172,000₫









  							WD29 FRESH BLOOM - 172,000₫









  							WD30 BLISS DAWN - 172,000₫
						
					
          
					
						
							
								
							
							
								
									
										
									
								
								
									[New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g
								
								
									299,000₫
									172,000₫
								
								
									
										
											Số lượng: 17
										
										
											Thông tin Outlet
										
									

								
								
						
						
							




	
		Tiêu đề
	
	
		
		 
		
		
		 
		
		
		

			
			
				
				WD14 WITH ZEST 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD20 MISTY WOOD 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD21 BURNT MAPLE 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD22 HAZEL CHILI 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD23 ANTIQUE FLAME 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD26 HUSHED APPEAL 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD27 FIG DUSTY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD28 WHISPER ODDY 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD29 FRESH BLOOM 
			
			
		
		
		 
		
		
		 
		
		
		

			
			
				
				WD30 BLISS DAWN 
			
			
		
		
	


							
								Vui lòng chọn biến thể
							
						
						
							Số lượng
							
								-
								
								+
							
						
					 
          
					
						WD14 WITH ZEST
						



    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							


    							
    								
    									
    								
    							
						
					
          

					

					
						
							Nhắc nhở tôi
						
            
  						
  							Mua Online
  						

						
							Thêm vào giỏ
						


						
							
								
									
								
								10/11 chi nhánh còn hàng
							
							
								
									
										Bạn vui lòng chọn biến thể
									
									
										
											
												ONLINE
											
											
												- Còn hàng (giao HOẢ TỐC 2H HCM, 24h toàn quốc)
												- Sắp về hàng(giao HOẢ TỐC 2H HCM, 24h toàn quốc)
											
										
										
										
											
												OFFLINE - CN Hồ Chí Minh
											
											
												
												
													
														
														Quận 3
													
													
														
														- Còn hàng  365 Lê Văn Sỹ, Phường 12
														
													 
												
												
												
													
														
														Quận 5
													
													
														
														- Còn hàng  159 Nguyễn Văn Cừ, Phường 2
														
													 
												
												
												
													
														
														Quận Tân Bình
													
													
														
														- Còn hàng  100 Hoàng Hoa Thám, Phường 12
														
													 
												
												
												
													
														
														Quận Bình Thạnh
													
													
														
														- Còn hàng  94 D5, Phường 25
														
													 
												
												
												
													
														
														Quận Gò Vấp
													
													
														
														- Còn hàng  15 Phan Văn Trị, phường 7
														
													 
												
												
												
													
														
														TP. Thủ Đức
													
													
														
														- Còn hàng  366 Võ Văn Ngân, phường Bình Thọ
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN MỸ THO - TIỀN GIANG
											
											
												
												
													
														
														TP Mỹ Tho - Tiền Giang
													
													
														
														- Sắp về hàng  Tầng L1, TTTM Vincom Plaza, 1A Hùng Vương, Phường 1
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN ĐỒNG NAI - BIÊN HÒA
											
											
												
												
													
														
														TP Biên Hòa - Đồng Nai
													
													
														
														- Còn hàng  278 Võ Thị Sáu, phường Thống Nhất
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HÀ NỘI
											
											
												
												
													
														
														NAM TỪ LIÊM
													
													
														
														- Còn hàng  240 Nguyễn Trãi, phường Trung Văn (cách Đại học Hà Nội 50m)
														
													 
												
												
											
										
										
										
											
												OFFLINE - CN HUẾ
											
											
												
												
													
														
														TP.Huế - Thừa Thiên Huế
													
													
														
														- Còn hàng  Tầng 1, TTTM  AEON MALL, 08 Võ Nguyên Giáp, phường An Đông
														
													 
												
												
											
										
										
									
								
							
						
					

          

          
            
              
                                QUÀ TẶNG KHUYẾN MÃI                     
                        
                          
                            
                              
                            
                          
                          
                            MUA 1 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 1 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Cột Tóc Merzy New Dew Scrunchies (màu ngẫu nhiên) - Trị Giá 79k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            79,000₫
                            Giảm còn: 0₫
                            79,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Túi Phao Make Up Merzy New Dew Bag (màu ngẫu nhiên) - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                      
                        
                          
                            
                              
                            
                          
                          
                            MUA 2 TẶNG 1 x [Gift] Lịch Để Bàn Merzy - Trị Giá 150k - khi mua 2 x [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g - 
                          
                          
                            150,000₫
                            Giảm còn: 0₫
                            150,000₫
                          
                        
                                         
          
          
							
  
    
      Các sản phẩm được mua kèm						
      Chọn 1 trong các loại sản phẩm sau
    							
    
      									
        MUA KÈMMua 1 [New Dew] Son Tint Bóng Merzy Dạng Thạch, Bền Màu, Lâu Trôi Merzy New Watery Dew Tint 4g để nhận ngay ưu đãi đi kèm (Tối đa 3)Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Hồng15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Dương15,000₫Túi Hộp Quà Tặng Nhỏ Enjoy The Sweetness.Xanh Lá15,000₫Mua ngay
      
    
  


	
		Chương trình khuyến mãi
		

		
	


	
		Khuyến mãi
	 
	 
											
			
		
	

          
  					
  						var price=172000;
  						
  						   Mua trả sau 0 ₫ với                                                                 .cls-1 { fill:#212121; } .cls-2 { fill:#fff; }               .promo-cls-1 { fill:#fff; }            Giảm đến 50K khi thanh toán qua Fundiin. Xem thêm     ×     
  						
              
              
              
              
  							
  								Miễn phí thanh toán
  								
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
                      
                      
                      
                      
                      
      									
      										
      									
                      
                    
  									
  										Chi tiết 
  											
  												
  											
  										
  									
  								
  							
  							
  								
  									Ẩn bớt
  										
  											
  										
  									
  								
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví VNPAY - Miễn phí thanh toán
      										Nhập mã: VNPAYSKIN25  Giảm 10K cho đơn hàng từ 399K  Áp dụng 14/03/2025 - 30/06/2025
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán qua Fundiin - Mua trước trả sau
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ShopeePay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví ZaloPay - Miễn phí thanh toán
      										
      									
      								
                    
                  
                    
                    
                    
                    
                    
      								
      									
      										
      									
      									
      										Thanh toán ví Momo - Miễn phí thanh toán
      										
      									
      								
                    
                  
  							
    					
  					

					
          
				&quot;))]</value>
      <webElementGuid>c669493e-b4f6-414b-bbdc-fa3df3d75b23</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
